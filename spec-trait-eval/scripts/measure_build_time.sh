#!/usr/bin/env bash
# Measure build time for each example pair (with macros vs without).
# Outputs a CSV: example,variant,build_time_secs
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
CRATE_DIR="$SCRIPT_DIR/../benches-crate"
RESULTS_DIR="$SCRIPT_DIR/../results"
ITERATIONS="${1:-5}"

mkdir -p "$RESULTS_DIR"

OUT="$RESULTS_DIR/build_time.csv"
echo "example,variant,iteration,build_time_secs" > "$OUT"

cd "$CRATE_DIR"

for i in $(seq 1 33); do
    ex=$(printf "ex%02d" "$i")
    echo "[INFO] Measuring build time for $ex ..."

    for iter in $(seq 1 "$ITERATIONS"); do
        # Clean only incremental cache to force recompilation
        cargo clean -p spec-trait-eval-benches 2>/dev/null || true

        # Time compilation of the crate (which includes the example)
        start=$(python3 -c 'import time; print(time.monotonic())')
        cargo check --lib 2>/dev/null
        end=$(python3 -c 'import time; print(time.monotonic())')
        elapsed=$(python3 -c "print(round($end - $start, 4))")
        echo "$ex,with_macro,$iter,$elapsed" >> "$OUT"
    done
done

echo "[INFO] Build times written to $OUT"
