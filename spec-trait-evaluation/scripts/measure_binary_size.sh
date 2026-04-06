#!/usr/bin/env bash
# Measure per-example artifact sizes for all benchmark variants.
# Outputs CSV: example,variant,binary_size_bytes,stripped_size_bytes
#
# Usage:
#   ./measure_binary_size.sh [max_example]
# Example:
#   ./measure_binary_size.sh 8
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
EXAMPLES_DIR="$SCRIPT_DIR/../benches-crate/src/examples"
RESULTS_DIR="$SCRIPT_DIR/../results"
MAX_EXAMPLE="${1:-8}"

mkdir -p "$RESULTS_DIR"

OUT="$RESULTS_DIR/binary_size.csv"
echo "example,variant,binary_size_bytes,stripped_size_bytes" > "$OUT"

VARIANTS=(spec naive runtime_typeid)

TMP_CRATE="$(mktemp -d -t spec-trait-bin-size-XXXXXX)"
trap 'rm -rf "$TMP_CRATE"' EXIT

MACRO_CRATE_PATH="$(cd "$SCRIPT_DIR/../../spec-trait-impl/crates/spec-trait-macro" && pwd)"
ORDER_CRATE_PATH="$(cd "$SCRIPT_DIR/../../spec-trait-impl/crates/spec-trait-order" && pwd)"

cat > "$TMP_CRATE/Cargo.toml" <<EOF
[package]
name = "spec-trait-size-probe"
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

# Warmup: pre-compile all dependencies once (release mode)
echo "fn warmup() {}" > "$TMP_CRATE/src/lib.rs"
cargo build --manifest-path "$TMP_CRATE/Cargo.toml" --lib --release --quiet 2>/dev/null || true

size_of_variant() {
    local file_path="$1"

    cp "$file_path" "$TMP_CRATE/src/subject.rs"

    cat > "$TMP_CRATE/src/lib.rs" <<EOF
#![allow(dead_code, unused_imports)]
mod subject;
pub use subject::*;
EOF

    # Remove only local crate artifacts, keep dependency cache
    rm -rf "$TMP_CRATE"/target/release/deps/libspec_trait_size_probe* \
           "$TMP_CRATE"/target/release/deps/spec_trait_size_probe* \
           "$TMP_CRATE"/target/release/.fingerprint/spec-trait-size-probe-* \
           "$TMP_CRATE"/target/release/libspec_trait_size_probe* \
           "$TMP_CRATE"/target/release/incremental/spec_trait_size_probe-* \
           2>/dev/null || true
    cargo build --manifest-path "$TMP_CRATE/Cargo.toml" --lib --release --quiet

    local rlib
    rlib=$(ls -t "$TMP_CRATE"/target/release/deps/libspec_trait_size_probe*.rlib 2>/dev/null | head -1)
    if [[ -z "$rlib" || ! -f "$rlib" ]]; then
        echo "0,0"
        return
    fi

    local size
    size=$(stat -f%z "$rlib" 2>/dev/null || stat --printf="%s" "$rlib" 2>/dev/null)
    echo "$size,$size"
}

for i in $(seq 1 "$MAX_EXAMPLE"); do
    ex=$(printf "ex%02d" "$i")
    echo "[INFO] Measuring artifact size for $ex ..."

    for variant in "${VARIANTS[@]}"; do
        src_file="$EXAMPLES_DIR/${ex}_${variant}.rs"
        if [[ ! -f "$src_file" ]]; then
            continue
        fi
        vals=$(size_of_variant "$src_file")
        size="${vals%%,*}"
        strip="${vals##*,}"
        echo "$ex,$variant,$size,$strip" >> "$OUT"
    done
done

echo "[INFO] Binary/artifact sizes written to $OUT"
