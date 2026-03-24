# spec-trait-eval

Evaluation infrastructure for **Meta-Monomorphizing Specializations**.

## Structure

```
spec-trait-eval/
├── benches-crate/          # Rust crate with 20 paired benchmarks
│   ├── benches/bench_all.rs    # Criterion benchmark harness
│   ├── src/
│   │   ├── lib.rs
│   │   ├── data.rs             # Test data generators
│   │   └── examples/
│   │       ├── ex01.rs … ex20.rs       # WITH spec!/when macros
│   │       └── ex01_wo.rs … ex20_wo.rs # WITHOUT macros (standard approaches)
│   ├── build.rs
│   └── Cargo.toml
├── scripts/
│   ├── run_all.sh              # Run all benchmarks, collect CSV
│   ├── measure_build_time.sh   # Measure compilation overhead
│   ├── measure_binary_size.sh  # Measure release binary sizes
│   └── collect_metrics.py      # Parse Criterion JSON → CSV
├── notebooks/
│   └── eval.ipynb              # Analysis & paper-ready figures
└── results/                    # Generated output (gitignored)
```

## Quick Start

```bash
# 1. Run runtime benchmarks (takes ~15–30 min)
cd benches-crate
cargo bench --bench bench_all

# 2. Collect results into CSV
cd ../scripts
python3 collect_metrics.py \
    --criterion-dir ../benches-crate/target/criterion \
    --output ../results/runtime.csv

# 3. Open the notebook
cd ../notebooks
jupyter lab eval.ipynb
```

## 4-Pillar Evaluation

| Pillar | What | How |
|--------|------|-----|
| **Runtime** | Latency per operation | Criterion benchmarks at 5 input sizes |
| **Build Time** | Compilation overhead | `measure_build_time.sh` |
| **Binary Size** | Object code footprint | `measure_binary_size.sh` |
| **Maintainability** | Lines of code | LOC comparison in notebook |

## Examples

Each example benchmarks a specific operation with up to 5 approaches:

| # | Operation | Specialization |
|---|-----------|---------------|
| 01 | Saturating double | `T = u8`, `T = i32` |
| 02 | Bitcount / popcount | `T = u8`, `T = u32`, `T = u64` |
| 03 | Sum | `T = i32`, `T = f64`, `T = u8` |
| 04 | Absolute value | `T = i32`, `T = f64`, `T = i64` |
| 05 | Serialize to bytes | `T = i32`, `T = f64`, `T = String` |
| 06 | ToString (itoa/ryu) | `T = i32`, `T = u64`, `T = f64` |
| 07 | FNV-1a hash | `T = u8`, `T = i32` |
| 08 | Bulk serialization | `T = u8`, `T = i32` |
| 09 | Copy vs Clone | `T: Copy` (trait bound) |
| 10 | Dedup | `T = u8`, `T = i32` |
| 11 | Map / transform | `T = i32`, `T = f64`, `T = u8` |
| 12 | Sort (counting) | `T = u8`, `T = i32` |
| 13 | Format (itoa/ryu) | `T = i32`, `T = f64`, `T = String` |
| 14 | Min / Max | `T = u8`, `T = i32`, `T = f64` |
| 15 | Contains / search | `T = u8` (memchr), `T = i32` |
| 16 | Fill / memset | `T = u8`, `T = i32` |
| 17 | Clamping | `T = i32`, `T = f64`, `T = u8` |
| 18 | FMA | `T = f64`, `T = f32`, `T = i32` |
| 19 | Byte conversion | `T = u8`, `T = char`, `T = i32` |
| 20 | Dot product | `T = f64`, `T = f32`, `T = i32` |

## Approaches Compared

- **spec** — `spec!` macro: compile-time specialization dispatch (zero runtime cost)
- **generic** — Default generic trait implementation (no specialization)
- **enum** — Manual enum dispatch with match arms
- **runtime_typeid** — Runtime `TypeId::of` checks
- **dyn** — Dynamic dispatch via `&dyn Trait`
