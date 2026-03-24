#!/usr/bin/env bash
# Measure per-example compile time for with/without macro variants.
# Outputs CSV: example,variant,iteration,build_time_secs
#
# Usage:
#   ./measure_build_time.sh [iterations] [max_example]
# Example:
#   ./measure_build_time.sh 5 33
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
EXAMPLES_DIR="$SCRIPT_DIR/../benches-crate/src/examples"
RESULTS_DIR="$SCRIPT_DIR/../results"
ITERATIONS="${1:-5}"
MAX_EXAMPLE="${2:-33}"

mkdir -p "$RESULTS_DIR"

OUT="$RESULTS_DIR/build_time.csv"
echo "example,variant,iteration,build_time_secs" > "$OUT"

TMP_CRATE="$(mktemp -d -t spec-trait-build-time-XXXXXX)"
trap 'rm -rf "$TMP_CRATE"' EXIT

MACRO_CRATE_PATH="$(cd "$SCRIPT_DIR/../../spec-trait-impl/crates/spec-trait-macro" && pwd)"
ORDER_CRATE_PATH="$(cd "$SCRIPT_DIR/../../spec-trait-impl/crates/spec-trait-order" && pwd)"

cat > "$TMP_CRATE/Cargo.toml" <<EOF
[package]
name = "spec-trait-compile-probe"
version = "0.1.0"
edition = "2024"

[dependencies]
spec-trait-macro = { path = "$MACRO_CRATE_PATH" }
itoa = "1"
ryu = "1"
memchr = "2"

[build-dependencies]
spec-trait-order = { path = "$ORDER_CRATE_PATH" }
EOF

mkdir -p "$TMP_CRATE/src"

cat > "$TMP_CRATE/build.rs" <<'EOF'
fn main() {
    spec_trait_order::handle_order();
}
EOF

measure_variant() {
    local file_path="$1"

    cp "$file_path" "$TMP_CRATE/src/subject.rs"

    cat > "$TMP_CRATE/src/lib.rs" <<EOF
#![allow(dead_code, unused_imports)]
mod subject;
pub use subject::*;
EOF

    cargo clean --manifest-path "$TMP_CRATE/Cargo.toml" >/dev/null 2>&1 || true

    python3 - "$TMP_CRATE" <<'PY'
import subprocess
import sys
import time

crate = sys.argv[1]
start = time.perf_counter()
proc = subprocess.run(
    ["cargo", "check", "--manifest-path", f"{crate}/Cargo.toml"],
    stdout=subprocess.PIPE,
    stderr=subprocess.PIPE,
    text=True,
)
if proc.returncode != 0:
    if proc.stdout:
        print(proc.stdout, file=sys.stderr, end="")
    if proc.stderr:
        print(proc.stderr, file=sys.stderr, end="")
    raise subprocess.CalledProcessError(proc.returncode, proc.args)
end = time.perf_counter()
print(f"{end - start:.6f}")
PY
}

for i in $(seq 1 "$MAX_EXAMPLE"); do
    ex=$(printf "ex%02d" "$i")
    with_file="$EXAMPLES_DIR/${ex}.rs"
    without_file="$EXAMPLES_DIR/${ex}_wo.rs"

    if [[ ! -f "$with_file" || ! -f "$without_file" ]]; then
        echo "[WARN] Skipping $ex: pair not found"
        continue
    fi

    echo "[INFO] Measuring build time for $ex (with/without)..."

    for iter in $(seq 1 "$ITERATIONS"); do
        with_t=$(measure_variant "$with_file")
        without_t=$(measure_variant "$without_file")
        delta=$(python3 - <<PY
with_t = float("$with_t")
without_t = float("$without_t")
print(f"{with_t - without_t:.6f}")
PY
)

        echo "$ex,with_macro,$iter,$with_t" >> "$OUT"
        echo "$ex,without_macro,$iter,$without_t" >> "$OUT"
        echo "$ex,delta_with_minus_without,$iter,$delta" >> "$OUT"
    done
done

echo "[INFO] Build times written to $OUT"
