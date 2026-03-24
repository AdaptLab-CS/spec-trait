#!/usr/bin/env bash
# Measure binary size for each example compiled standalone.
# Outputs CSV: example,variant,binary_size_bytes,stripped_size_bytes
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
CRATE_DIR="$SCRIPT_DIR/../benches-crate"
RESULTS_DIR="$SCRIPT_DIR/../results"

mkdir -p "$RESULTS_DIR"

OUT="$RESULTS_DIR/binary_size.csv"
echo "example,variant,binary_size_bytes,stripped_size_bytes" > "$OUT"

cd "$CRATE_DIR"

# Build the bench binary in release mode
echo "[INFO] Building bench binary in release mode..."
cargo build --benches --release 2>/dev/null

# Find the bench binary
BENCH_BIN=$(find target/release -maxdepth 1 -name "bench_all*" -type f ! -name "*.d" | head -1)
if [[ -z "$BENCH_BIN" ]]; then
    echo "[WARN] Bench binary not found, trying deps directory..."
    BENCH_BIN=$(find target/release/deps -name "bench_all*" -type f ! -name "*.d" ! -name "*.o" | head -1)
fi

if [[ -n "$BENCH_BIN" ]]; then
    SIZE=$(stat -f%z "$BENCH_BIN" 2>/dev/null || stat --printf="%s" "$BENCH_BIN" 2>/dev/null)
    # Create a stripped copy
    cp "$BENCH_BIN" /tmp/bench_stripped
    strip /tmp/bench_stripped 2>/dev/null || true
    STRIPPED=$(stat -f%z /tmp/bench_stripped 2>/dev/null || stat --printf="%s" /tmp/bench_stripped 2>/dev/null)
    echo "bench_all,with_macro,$SIZE,$STRIPPED" >> "$OUT"
    rm -f /tmp/bench_stripped
    echo "[INFO] bench_all: ${SIZE} bytes (stripped: ${STRIPPED})"
fi

# Also build the lib in release to get .rlib sizes
echo "[INFO] Building lib in release mode..."
cargo build --lib --release 2>/dev/null

for rlib in target/release/deps/libspec_trait_eval_benches*.rlib; do
    if [[ -f "$rlib" ]]; then
        SIZE=$(stat -f%z "$rlib" 2>/dev/null || stat --printf="%s" "$rlib" 2>/dev/null)
        echo "lib,rlib,$SIZE,$SIZE" >> "$OUT"
        echo "[INFO] rlib: ${SIZE} bytes"
    fi
done

echo "[INFO] Binary sizes written to $OUT"
