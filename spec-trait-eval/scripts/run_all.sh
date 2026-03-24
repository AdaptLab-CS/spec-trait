#!/usr/bin/env bash
# Run all Criterion benchmarks and collect CSV output.
# Usage: ./run_all.sh [--quick]
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
CRATE_DIR="$SCRIPT_DIR/../benches-crate"
RESULTS_DIR="$SCRIPT_DIR/../results"

mkdir -p "$RESULTS_DIR"

EXTRA_ARGS=()
if [[ "${1:-}" == "--quick" ]]; then
    EXTRA_ARGS+=(--warm-up-time 1 --measurement-time 3 --sample-size 10)
    echo "[INFO] Quick mode: reduced iterations"
fi

echo "[INFO] Running benchmarks (this may take a while)..."
cd "$CRATE_DIR"
cargo bench --bench bench_all -- \
    --output-format bencher \
    "${EXTRA_ARGS[@]}" \
    2>&1 | tee "$RESULTS_DIR/raw_output.txt"

# Criterion saves detailed CSV in target/criterion/<group>/<variant>/new/estimates.json
echo "[INFO] Collecting Criterion results..."
python3 "$SCRIPT_DIR/collect_metrics.py" \
    --criterion-dir "$CRATE_DIR/target/criterion" \
    --output "$RESULTS_DIR/runtime.csv"

echo "[INFO] Results written to $RESULTS_DIR/runtime.csv"
