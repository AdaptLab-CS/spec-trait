#!/usr/bin/env bash
# Measure per-example artifact sizes for with/without macro variants.
# Outputs CSV: example,variant,binary_size_bytes,stripped_size_bytes
#
# Note: these are per-example rlib artifact sizes (not Criterion executable size).
# Usage:
#   ./measure_binary_size.sh [max_example]
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
EXAMPLES_DIR="$SCRIPT_DIR/../benches-crate/src/examples"
RESULTS_DIR="$SCRIPT_DIR/../results"
MAX_EXAMPLE="${1:-33}"

mkdir -p "$RESULTS_DIR"

OUT="$RESULTS_DIR/binary_size.csv"
echo "example,variant,binary_size_bytes,stripped_size_bytes" > "$OUT"

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

size_of_variant() {
    local file_path="$1"

    cp "$file_path" "$TMP_CRATE/src/subject.rs"

    cat > "$TMP_CRATE/src/lib.rs" <<EOF
#![allow(dead_code, unused_imports)]
mod subject;
pub use subject::*;
EOF

    cargo clean --manifest-path "$TMP_CRATE/Cargo.toml" >/dev/null 2>&1 || true
    cargo build --manifest-path "$TMP_CRATE/Cargo.toml" --lib --release --quiet

    local rlib
    rlib=$(ls -t "$TMP_CRATE"/target/release/deps/libspec_trait_size_probe*.rlib 2>/dev/null | head -1)
    if [[ -z "$rlib" || ! -f "$rlib" ]]; then
        echo "0,0"
        return
    fi

    local size
    size=$(stat -f%z "$rlib" 2>/dev/null || stat --printf="%s" "$rlib" 2>/dev/null)

    # rlib is an archive; keep stripped == size for schema compatibility
    echo "$size,$size"
}

for i in $(seq 1 "$MAX_EXAMPLE"); do
    ex=$(printf "ex%02d" "$i")
    with_file="$EXAMPLES_DIR/${ex}.rs"
    without_file="$EXAMPLES_DIR/${ex}_wo.rs"

    if [[ ! -f "$with_file" || ! -f "$without_file" ]]; then
        echo "[WARN] Skipping $ex: pair not found"
        continue
    fi

    echo "[INFO] Measuring artifact size for $ex (with/without)..."

    with_vals=$(size_of_variant "$with_file")
    without_vals=$(size_of_variant "$without_file")

    with_size="${with_vals%%,*}"
    with_strip="${with_vals##*,}"
    without_size="${without_vals%%,*}"
    without_strip="${without_vals##*,}"

    delta=$(python3 - <<PY
print(int("$with_size") - int("$without_size"))
PY
)

    echo "$ex,with_macro,$with_size,$with_strip" >> "$OUT"
    echo "$ex,without_macro,$without_size,$without_strip" >> "$OUT"
    echo "$ex,delta_with_minus_without,$delta,$delta" >> "$OUT"
done

echo "[INFO] Binary/artifact sizes written to $OUT"
