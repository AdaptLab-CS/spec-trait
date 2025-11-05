#!/usr/bin/env bash
set -uo pipefail

# Usage:
#   ./scripts/run-on-repos.sh -f repos.txt -c "cargo test --all" \
#       -w ./workspaces -r ./results [-k]
#
# repos.txt can contain one URL per line. Blank lines and lines starting with #
# are ignored.
#
# -f file with URLs (required)
# -c cargo command to run inside each repo (defaults to "cargo test")
# -w workdir where repos are cloned (default: ./workspaces, relative to tests/)
# -r results dir for logs and summary (default: ./results, relative to tests/)
# -k keep clones (do not remove repo dirs)

usage() {
  sed -n '1,200p' <<'USAGE'
Usage: run-on-repos.sh -f <file> [-c <cargo-cmd>] [-w <workdir>] [-r <results>] [-k]
  -f file with newline-separated repo URLs (required)
  -c cargo command to run inside each repo (default: "cargo build")
  -w workdir where repos are cloned (default: ./workspaces, relative to tests/)
  -r results directory for logs and summary (default: ./results, relative to tests/)
  -k keep clones (do not remove them)

This script expects to be run from the tests/ directory:
  cd ./spec-trait-inst/tests
  ./scripts/run-on-repos.sh -f repos.txt
USAGE
  exit 1
}

# ensure we're running from the tests/ directory
if [[ "$(basename "$PWD")" != "tests" ]]; then
  echo "Error: this script must be run from the tests/ directory (pwd: $PWD)" >&2
  echo "cd into the tests/ directory and run ./scripts/run-on-repos.sh" >&2
  exit 2
fi

file=""
cmd="clear && cargo clean && cargo build --manifest-path ../../../Cargo.toml && RUST_LOG_STYLE=always RUST_LOG=trace LD_LIBRARY_PATH=$(rustc --print sysroot)/lib ../../../target/debug/cargo-spec-trait-inst --print-spec-targets"
base_dir="$PWD"
workdir="$base_dir/workspaces"
results="$base_dir/results"
keep=false

while getopts "f:c:w:r:kh" opt; do
  case "$opt" in
    f) file="$OPTARG" ;;
    c) cmd="$OPTARG" ;;
    w) workdir="$OPTARG" ;;
    r) results="$OPTARG" ;;
    k) keep=true ;;
    h) usage ;;
    *) usage ;;
  esac
done

if [[ -z "$file" ]]; then
  echo "Missing -f file" >&2
  usage
fi

if [[ ! -f "$file" ]]; then
  echo "File not found: $file" >&2
  exit 3
fi

mkdir -p "$workdir" "$results"

# remove any existing .log files in results before running
shopt -s nullglob
for f in "$results"/*.log; do
  rm -f -- "$f"
done
shopt -u nullglob

summary="$results/summary.csv"
echo "url,crate_name,functions,specializable_fns,specializable_traits" > "$summary"

# sanitize name helper
sanitize() {
  echo "$1" | sed -E 's/[^a-zA-Z0-9._-]/_/g'
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

  echo -e "[$i] ---\nCloning $url -> $target_dir"
  if ! git clone --depth 1 "$url" "$target_dir" >/dev/null 2>&1; then
    echo "Clone failed for $url, skipping."
    echo "\"$url\",\"$repo_name\",0,0,0" >> "$summary"
    continue
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

  # remove repo if needed 
  if ! $keep; then
    rm -rf "$target_dir"
  fi

  # return if error 
  if [[ $rc -ne 0 ]]; then
    echo "Error: command failed for $repo_name (exit $rc). Log: $log_file"
    # echo "Aborting due to error."
    # exit $rc
  fi

  # functions: line like "Found x functions"
  if grep -qE 'Found [0-9]+ functions' "$log_file" 2>/dev/null; then
    functions=$(grep -Eo 'Found [0-9]+ functions' "$log_file" 2>/dev/null | grep -Eo '[0-9]+' | awk '{s+=$1} END{print s+0}')
  else
    functions=0
  fi

  # specializable pairs: line like "Specializable pairs: x fns and y traits"
  if grep -qE 'Specializable pairs: [0-9]+ fns and [0-9]+ traits' "$log_file" 2>/dev/null; then
    read -r spec_fns spec_traits < <(
      grep -Eo 'Specializable pairs: [0-9]+ fns and [0-9]+ traits' "$log_file" 2>/dev/null \
        | awk '{f+=$3; t+=$6} END{print f+0, t+0}'
    )
  else
    spec_fns=0
    spec_traits=0
  fi

  # append parsed results to summary (one final line per repo)
  echo "\"$url\",\"$repo_name\",$functions,$spec_fns,$spec_traits" >> "$summary"
  echo "Finished $url"

  
done < "$file"

echo "All done. Summary: $summary"
