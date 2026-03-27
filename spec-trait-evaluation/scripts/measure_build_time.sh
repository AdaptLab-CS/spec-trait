#!/usr/bin/env bash
# Measure per-example compile time for all benchmark variants.
# Outputs CSV: example,variant,iteration,build_time_secs
#
# Usage:
#   ./measure_build_time.sh [iterations] [max_example]
# Example:
#   ./measure_build_time.sh 5 8
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
EXAMPLES_DIR="$SCRIPT_DIR/../benches-crate/src/examples"
RESULTS_DIR="$SCRIPT_DIR/../results"
ITERATIONS="${1:-5}"
MAX_EXAMPLE="${2:-8}"

mkdir -p "$RESULTS_DIR"

OUT="$RESULTS_DIR/build_time.csv"
echo "example,variant,iteration,build_time_secs" > "$OUT"

VARIANTS=(spec generic dyn enum runtime_typeid)

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

# Warmup: pre-compile all dependencies once
echo "fn warmup() {}" > "$TMP_CRATE/src/lib.rs"
cargo check --manifest-path "$TMP_CRATE/Cargo.toml" --quiet 2>/dev/null || true

measure_variant() {
    local file_path="$1"

    cp "$file_path" "$TMP_CRATE/src/subject.rs"

    cat > "$TMP_CRATE/src/lib.rs" <<EOF
#![allow(dead_code, unused_imports)]
mod subject;
pub use subject::*;
EOF

    # Remove only local crate artifacts, keep dependency cache
    rm -rf "$TMP_CRATE"/target/debug/deps/libspec_trait_compile_probe* \
           "$TMP_CRATE"/target/debug/deps/spec_trait_compile_probe* \
           "$TMP_CRATE"/target/debug/.fingerprint/spec-trait-compile-probe-* \
           "$TMP_CRATE"/target/debug/libspec_trait_compile_probe* \
           "$TMP_CRATE"/target/debug/incremental/spec_trait_compile_probe-* \
           2>/dev/null || true

    python3 - "$TMP_CRATE" <<'PY'
import subprocess, sys, time
crate = sys.argv[1]
start = time.perf_counter()
proc = subprocess.run(
    ["cargo", "check", "--manifest-path", f"{crate}/Cargo.toml"],
    stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True,
)
if proc.returncode != 0:
    if proc.stdout: print(proc.stdout, file=sys.stderr, end="")
    if proc.stderr: print(proc.stderr, file=sys.stderr, end="")
    raise subprocess.CalledProcessError(proc.returncode, proc.args)
end = time.perf_counter()
print(f"{end - start:.6f}")
PY
}

for i in $(seq 1 "$MAX_EXAMPLE"); do
    ex=$(printf "ex%02d" "$i")
    echo "[INFO] Measuring build time for $ex ..."

    for iter in $(seq 1 "$ITERATIONS"); do
        for variant in "${VARIANTS[@]}"; do
            src_file="$EXAMPLES_DIR/${ex}_${variant}.rs"
            if [[ ! -f "$src_file" ]]; then
                continue
            fi
            t=$(measure_variant "$src_file")
            echo "$ex,$variant,$iter,$t" >> "$OUT"
        done
    done
done

echo "[INFO] Build times written to $OUT"
