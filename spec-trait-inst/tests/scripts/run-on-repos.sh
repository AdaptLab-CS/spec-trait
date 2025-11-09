#!/usr/bin/env bash
set -uo pipefail

usage() {
  sed -n '1,200p' <<'USAGE'
Usage: run-on-repos.sh [-f <file>] [-c <cargo-cmd>] [-w <workdir>] [-r <results>] [-k]
  -f file with newline-separated repo URLs (default: ./scripts/run-on-repos.txt, relative to tests/)
  -c cargo command to run inside each repo (default: "cargo build")
  -w workdir where repos are cloned (default: ./workspaces, relative to tests/)
  -r results directory for logs and summary (default: ./results, relative to tests/)
  -d delete previous entries in the summary file (default: true)
  -k keep clones (default: false)
  -h show this help message and exit

This script expects to be run from the tests/ directory:
  cd ./spec-trait-inst/tests
  ./scripts/run-on-repos.sh
USAGE
  exit 1
}

# ensure we're running from the tests/ directory
if [[ "$(basename "$PWD")" != "tests" ]]; then
  echo "Error: this script must be run from the tests/ directory (pwd: $PWD)" >&2
  echo "cd into the tests/ directory and run ./scripts/run-on-repos.sh" >&2
  exit 2
fi

base_dir="$PWD"
file="$base_dir/scripts/run-on-repos.txt"
cmd="clear && cargo clean && cargo build --manifest-path ../../../Cargo.toml && RUST_LOG_STYLE=always RUST_LOG=trace LD_LIBRARY_PATH=$(rustc --print sysroot)/lib ../../../target/debug/cargo-spec-trait-inst --print-spec-targets -- --all-features"
workdir="$base_dir/workspaces"
results="$base_dir/results"
delete_summary=true
keep=false

while getopts "f:c:w:r:dkh" opt; do
  case "$opt" in
    f) file="$OPTARG" ;;
    c) cmd="$OPTARG" ;;
    w) workdir="$OPTARG" ;;
    r) results="$OPTARG" ;;
    d) delete_summary=false ;;
    k) keep=true ;;
    h) usage ;;
    *) usage ;;
  esac
done

if [[ ! -f "$file" ]]; then
  echo "File not found: $file" >&2
  exit 3
fi

mkdir -p "$workdir" "$results"

# remove any existing .log files in results before running
shopt -s nullglob
# remove old logs
for f in "$results"/*.log; do
  rm -f -- "$f"
done
shopt -u nullglob

# prepare summary file
summary="$results/summary.csv"
if $delete_summary && [[ -f "$summary" ]]; then
  echo "url,crate_name,members_count,crates_io_downloads,github_stars,analysis_total_time_seconds,analysis_peak_rss_MB,total_fns,total_traits,specializable_fn_pairs,specializable_trait_pairs,specializable_fns,specializable_traits,already_specialized_fn_pairs,already_specialized_trait_pairs,already_specialized_fns,already_specialized_traits,already_specializable_fn_pairs,already_specializable_trait_pairs,already_specializable_fns,already_specializable_traits,newly_specializable_fn_pairs,newly_specializable_trait_pairs,newly_specializable_fns,newly_specializable_traits,already_specializable_unspecialized_fn_pairs,already_specializable_unspecialized_trait_pairs,already_specializable_unspecialized_fns,already_specializable_unspecialized_traits,specializable_fns_from_macros,specializable_traits_from_macros,specializable_fns_from_traits,specializable_fns_without_traits" > "$summary"
fi

# sanitize name helper
sanitize() {
  echo "$1" | sed -E 's/[^a-zA-Z0-9._-]/_/g'
}

# Count workspace members declared in a Cargo.toml file at the repository root.
# Returns 0 when no [workspace] members are found or file missing.
count_workspace_members() {
  local toml_file="$1"
  if [[ ! -f "$toml_file" ]]; then
    echo 0
    return
  fi

  # If there's a [workspace] table, parse members. If not, but there's a [package] table,
  # treat it as a single-crate repository and return 1. Otherwise return 0.
  if grep -q '^\[workspace\]' "$toml_file"; then
    # Find the first 'members = [' occurrence after [workspace]
    start_line=$(awk 'BEGIN{in_ws=0} /^\[workspace\]/{in_ws=1} in_ws && /members[ \t]*=/ {print NR; exit}' "$toml_file" || true)
    if [[ -z "$start_line" ]]; then
      echo 0
      return
    fi

    # Extract from the members= line until the first closing ']' and count entries
    members_block=$(tail -n +"$start_line" "$toml_file" | awk '{print} /\]/{exit}')
    # Try inline form first: members = ["a","b"]
    inline=$(echo "$members_block" | tr -d '\n' | sed 's/.*\[//;s/\].*//')
    if [[ -n "$inline" ]]; then
      IFS=',' read -ra parts <<< "$inline"
      cnt=0
      for p in "${parts[@]}"; do
        # trim whitespace
        p="${p#"${p%%[![:space:]]*}"}"
        p="${p%"${p##*[![:space:]]}"}"
        # remove surrounding quotes if present
        if { [[ "${p:0:1}" == '"' && "${p: -1}" == '"' ]] || [[ "${p:0:1}" == "'" && "${p: -1}" == "'" ]]; }; then
          p="${p:1:-1}"
        fi
        [[ -n "$p" ]] && ((cnt++))
      done
      echo "$cnt"
      return
    fi

    # fallback: count quoted entries across multiple lines
    cnt=$(echo "$members_block" | grep -o '"[^"]\+"' | wc -l || true)
    echo "${cnt:-0}"
    return
  elif grep -q '^\[package\]' "$toml_file"; then
    # single-package crate
    echo 1
    return
  else
    echo 0
    return
  fi
}

# Extract owner/repo path from common GitHub URL formats.
# Accepts: https://github.com/owner/repo(.git), http(s) with trailing path, git@github.com:owner/repo.git
get_github_repo_path() {
  local url="$1"
  # We assume the input URL is a GitHub URL (https or ssh) and extract owner/repo.
  # Handles forms like:
  #   git@github.com:owner/repo.git
  #   https://github.com/owner/repo
  #   https://github.com/owner/repo.git
  #   https://github.com/owner/repo/tree/main
  if [[ "$url" =~ ^git@github.com:([^/]+/[^/]+)(\.git)?$ ]]; then
    echo "${BASH_REMATCH[1]}" | sed 's/\.git$//'
    return
  fi

  if [[ "$url" =~ ^https?://([^/]+)/(.*)$ ]]; then
    host="${BASH_REMATCH[1]}"
    path="${BASH_REMATCH[2]}"
    # only accept github.com host; if other host provided, still attempt to parse
    # take first two path segments as owner/repo
    owner=$(echo "$path" | cut -d'/' -f1)
    repo=$(echo "$path" | cut -d'/' -f2)
    repo=$(echo "$repo" | sed 's/\.git$//')
    if [[ -n "$owner" && -n "$repo" ]]; then
      echo "$owner/$repo"
      return
    fi
  fi

  # If nothing matched, return empty to indicate failure
  echo ""
}

# Query GitHub API for repo stars. Returns integer >=0 or 0 on failure.
# If GITHUB_TOKEN is set in the environment, use it for authenticated requests (higher rate limit).
get_github_stars() {
  local url="$1"
  local repo_path
  repo_path=$(get_github_repo_path "$url")
  if [[ -z "$repo_path" ]]; then
    echo 0
    return
  fi

  api_url="https://api.github.com/repos/$repo_path"
  if [[ -n "${GITHUB_TOKEN-}" ]]; then
    auth_header=( -H "Authorization: token ${GITHUB_TOKEN}" )
  else
    auth_header=()
  fi

  # Query API (silent). We only need the stargazers_count field.
  resp=$(curl -sS -H "Accept: application/vnd.github.v3+json" "${auth_header[@]}" "$api_url" || true)
  if [[ -z "$resp" ]]; then
    echo 0
    return
  fi
  # Extract stargazers_count using grep/sed
  stars=$(echo "$resp" | grep -o '"stargazers_count"[[:space:]]*:[[:space:]]*[0-9]\+' | grep -o '[0-9]\+' || true)
  if [[ -n "$stars" ]]; then
    echo "$stars"
    return
  fi

  # fallback 0
  echo 0
}

# Parse a Cargo.toml and return the package.name if present; empty string otherwise.
parse_package_name() {
  local toml_file="$1"
  if [[ ! -f "$toml_file" ]]; then
    echo ""
    return
  fi

  # Extract lines inside the [package] table (stop at next table)
  local pkg_block
  pkg_block=$(awk 'BEGIN{in_pkg=0} /^\[package\]/{in_pkg=1; next} in_pkg && /^\[/{exit} in_pkg{print}' "$toml_file" || true)
  if [[ -z "$pkg_block" ]]; then
    echo ""
    return
  fi

  # Look for a name = "..." or name = '...' line and capture the value.
  # Use sed with a double-quoted script to allow both quote types in the regex.
  local name
  name=$(printf '%s\n' "$pkg_block" \
    | sed -nE "s/^[[:space:]]*name[[:space:]]*=[[:space:]]*['\"]?([^'\"#[:space:]]+)['\"]?.*/\1/p" \
    | head -n1 || true)

  echo "${name:-}"
}

# Extract workspace member paths from a top-level Cargo.toml. Prints one path per line.
extract_workspace_member_paths() {
  local toml_file="$1"
  if [[ ! -f "$toml_file" ]]; then
    return
  fi
  start_line=$(awk 'BEGIN{in_ws=0} /^\[workspace\]/{in_ws=1} in_ws && /members[ \t]*=/ {print NR; exit}' "$toml_file" || true)
  if [[ -z "$start_line" ]]; then
    return
  fi
  members_block=$(tail -n +"$start_line" "$toml_file" | awk '{print} /\]/{exit}')
  inline=$(echo "$members_block" | tr -d '\n' | sed 's/.*\[//;s/\].*//')
  if [[ -n "$inline" ]]; then
      # split by comma using bash to avoid awk-quoting issues and print each member on its own line
      IFS=',' read -ra _parts <<< "$inline"
      for part in "${_parts[@]}"; do
        p="$part"
        # trim leading/trailing whitespace using bash parameter expansion
        p="${p#${p%%[![:space:]]*}}"
        p="${p%${p##*[![:space:]]}}"
        # remove surrounding quotes if both ends match (single or double)
        if { [[ "${p:0:1}" == '"' && "${p: -1}" == '"' ]] || [[ "${p:0:1}" == "'" && "${p: -1}" == "'" ]]; }; then
          p="${p:1:-1}"
        fi
        if [[ -n "$p" ]]; then
          printf '%s\n' "$p"
        fi
      done
    return
  fi
  # fallback: try to parse quoted entries across lines
  echo "$members_block" | grep -o '"[^"]\+"' | sed 's/"//g' || true
}

# Query crates.io for a crate name and return its downloads (integer), 0 on failure
get_crates_io_downloads_for_name() {
  local crate="$1"
  local api_url="https://crates.io/api/v1/crates/$crate"
  local user_agent="${CRATES_IO_USER_AGENT:-spec-trait-inst/1.0}"
  local resp

  resp=$(curl -sS -H "Accept: application/json" -H "User-Agent: $user_agent" "$api_url" || true)
  if [[ -z "$resp" ]]; then
    echo 0
    return
  fi

  # If API returned an errors object, treat as unavailable
  if echo "$resp" | grep -q '"errors"[[:space:]]*:'; then
    echo 0
    return
  fi

  # collapse JSON to a single line, then look for "crate" object and its "downloads" field
  one_line=$(echo "$resp" | tr -d '\n' | sed 's/[[:space:]]\+/ /g')

  # pick the first "downloads" anywhere in the JSON
  downloads=$(echo "$one_line" | grep -o '"downloads"[[:space:]]*:[[:space:]]*[0-9]\+' | head -n1 | grep -o '[0-9]\+' || true)
  if [[ -n "$downloads" ]]; then
    echo "$downloads"
  else
    echo 0
  fi
}

# Get total crates.io downloads for a repository at path (target_dir).
# Sums downloads for package in top-level Cargo.toml or for all workspace members.
get_crates_io_downloads() {
  local repo_dir="$1"
  local fallback_name="${2-}"
  local top_manifest="$repo_dir/Cargo.toml"
  if [[ ! -f "$top_manifest" ]]; then
    echo 0
    return
  fi
  # if top-level package, use that
  pkg=$(parse_package_name "$top_manifest" | tr -d '\n')
  if [[ -n "$pkg" ]]; then
    get_crates_io_downloads_for_name "$pkg"
    return
  fi
  # fallback: if no package name found, use provided fallback repo name as crate name
  if [[ -n "$fallback_name" ]]; then
    get_crates_io_downloads_for_name "$fallback_name"
    return
  fi
  # else, try workspace members
  sum=0
  while IFS= read -r member; do
    # member paths are relative to repo_dir
    member_path=$(echo "$member" | sed 's#^/*##')
    manifest_path="$repo_dir/$member_path/Cargo.toml"
    # if manifest exists, parse package name
    if [[ -f "$manifest_path" ]]; then
      name=$(parse_package_name "$manifest_path" | tr -d '\n')
      if [[ -n "$name" ]]; then
        d=$(get_crates_io_downloads_for_name "$name")
        sum=$((sum + d))
      fi
    else
      # fallback: assume member name is last path segment
      name=$(basename "$member_path")
      d=$(get_crates_io_downloads_for_name "$name")
      sum=$((sum + d))
    fi
  done < <(extract_workspace_member_paths "$top_manifest")
  echo "$sum"
}

i=0
while IFS= read -r raw || [[ -n "$raw" ]]; do
  line="${raw%%#*}"            # strip trailing comment
  line="${line#"${line%%[![:space:]]*}"}"  # trim left
  line="${line%"${line##*[![:space:]]}"}"  # trim right
  [[ -z "$line" ]] && continue

  url="$line"
  i=$((i+1))

  # derive repo name from URL
  base="$(basename "${url%%\?*}")"
  repo_name="${base%.git}"
  repo_name="$(sanitize "$repo_name")"
  # ensure unique
  target_dir="$workdir/$repo_name"
  if [[ -e "$target_dir" ]]; then
    suffix=1
    while [[ -e "${target_dir}-${suffix}" ]]; do
      suffix=$((suffix+1))
    done
    target_dir="${target_dir}-${suffix}"
  fi

  echo "[$i] $repo_name"
  echo "Cloning from $url"
  if ! git clone --depth 1 "$url" "$target_dir" >/dev/null 2>&1; then
    echo "Error: clone failed for $repo_name (exit 4)"
    echo "Aborting due to error."
    exit 4
  fi

  # remove existing rust-toolchain.toml to avoid toolchain conflicts
  if [[ -f "$target_dir/rust-toolchain.toml" ]]; then
    rm "$target_dir/rust-toolchain.toml"
  fi

  # run the cargo command
  log_file="$results/${repo_name}.log"
  echo "Running in $target_dir"
  pushd "$target_dir" >/dev/null
  # run command, capture exit code
  # Use bash -lc to allow shell features in cmd
  if bash -lc "$cmd" > "$log_file" 2>&1; then
    rc=0
  else
    rc=$?
  fi
  popd >/dev/null

  # return if error 
  if [[ $rc -ne 0 ]]; then
    echo "Error: command failed for $repo_name (exit $rc)"
    echo "Aborting due to error."
    exit $rc
  fi

  # count workspace members in the repo's top-level Cargo.toml (0 if none)
  manifest="$target_dir/Cargo.toml"
  members_count=$(count_workspace_members "$manifest")
  # query GitHub stars (0 if unavailable)
  github_stars=$(get_github_stars "$url")
  # query crates.io downloads (sum for workspace or single crate)
  crates_downloads=$(get_crates_io_downloads "$target_dir" "$repo_name")

  # get time and memory usage from log ("STIAnalysis for crate $repo_name completed in $elapsed_time s with peak rss $peak_rss MB")
  # collect all occurrences (one per member)
  mapfile -t _time_lines < <(grep -oP 'STIAnalysis for crate .* completed in \K[0-9.]+' "$log_file" || true)
  mapfile -t _rss_lines < <(grep -oP 'STIAnalysis for crate .* with peak rss \K[0-9]+' "$log_file" || true)
  total_time=0
  peak_rss=0
  for t in "${_time_lines[@]}"; do
    total_time=$(echo "$total_time + $t" | bc)
  done
  for r in "${_rss_lines[@]}"; do
    # skip empty entries
    [[ -z "$r" ]] && continue
    if (( r > peak_rss )); then
      peak_rss=$r
    fi
  done

  # get specialization analysis results from log ("Specialization analysis data: $specialization_data")
  # collect all occurrences (one per member) and sum each comma-separated column
  mapfile -t _spec_lines < <(grep -oP 'Specialization analysis data: \K.*' "$log_file" || true)
  if [[ ${#_spec_lines[@]} -eq 0 ]]; then
    specialization_data=""
  else
    declare -a _sums=()
    for _line in "${_spec_lines[@]}"; do
      # split on commas
      IFS=',' read -ra _parts <<< "$_line"
      for idx in "${!_parts[@]}"; do
        p="${_parts[idx]}"
        # trim whitespace
        p="${p#"${p%%[![:space:]]*}"}"
        p="${p%"${p##*[![:space:]]}"}"
        # default empty->0 and ensure numeric (non-numeric treated as 0)
        if [[ -z "$p" || ! "$p" =~ ^-?[0-9]+$ ]]; then
          p=0
        fi
        _sums[idx]=$(( ${_sums[idx]:-0} + p ))
      done
    done
    # rebuild comma-separated list (preserve order)
    specialization_data="$(IFS=,; echo "${_sums[*]}")"
  fi

  # append parsed results to summary (one final line per repo)
  echo "\"$url\",\"$repo_name\",$members_count,$crates_downloads,$github_stars,$total_time,$peak_rss,$specialization_data" >> "$summary"

  # remove repo if needed
  if ! $keep; then
    rm -rf "$target_dir"
  fi
  
done < "$file"

echo "All done. Summary: $summary"