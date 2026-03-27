# spec-trait-eval

Evaluation infrastructure for **Meta-Monomorphizing Specializations**.

## Structure

```
spec-trait-eval/
├── benches-crate/          # Rust crate with benchmark examples
│   ├── benches/bench_all.rs    # Criterion benchmark harness
│   ├── src/
│   │   ├── lib.rs
│   │   ├── data.rs             # Test data generators
│   │   └── examples/
│   │       ├── ex03_spec.rs … ex41_spec.rs # spec implementations
│   │       ├── exNN_compat.rs              # unsplit non-spec source (kept for reference)
│   │       └── exNN_{generic,dyn,enum,runtime_typeid}.rs # split non-spec variants
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
# or to run a specific example:
cargo bench --bench bench_all example_name 
# or for more control:
cargo bench --bench bench_all -- --warmup-time 0s --measurement-time 30s

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
| **Build Time** | Compilation overhead (per example, with vs without macros) | `measure_build_time.sh` |
| **Binary Size** | Per-example compiled artifact footprint (with vs without macros) | `measure_binary_size.sh` |
| **Maintainability** | Lines of code | LOC comparison in notebook |

## Script Inputs And Outputs

The three measurement scripts are independent (they do not call each other):

| Script | Reads | Writes |
|--------|-------|--------|
| `collect_metrics.py` | `benches-crate/target/criterion/**/new/estimates.json` (Criterion runtime results) | `results/runtime.csv` |
| `measure_build_time.sh` | Per-example source pairs `exNN.rs` / `exNN_compat.rs`, compiled in a probe crate | `results/build_time.csv` |
| `measure_binary_size.sh` | Per-example source pairs `exNN.rs` / `exNN_compat.rs`, built as release `rlib` in a probe crate | `results/binary_size.csv` |

Notes:
- `collect_metrics.py` does not read `build_time.csv` or `binary_size.csv`.
- To get a full 4-pillar analysis, run all scripts and combine their CSVs in the notebook.
- In `runtime.csv`, `size` means input size (number of processed elements), not binary size.
- In `build_time.csv`, `variant=delta_with_minus_without` can be negative: this means `with_macro` compiled faster than `without_macro` for that example/iteration.
- In `binary_size.csv`, rows are per-example artifact sizes (`with_macro`, `without_macro`) plus `delta_with_minus_without`.

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
| 21 | Lifetime parsing | `T = &'static str` vs `T = &'a str` (lifetime specialization) |
| 22 | HRTB apply | `F = for<'a> fn(&'a T) -> T` (higher-ranked trait bounds) |
| 23 | Multi-generic combine | `U = Vec<T>`, `T = Vec<U>` (repeating generics) |
| 24 | Wildcard measure | `Vec<_>`, `(i32, _)`, `&[i32]`, `fn(&u8)->&u8` (complex types) |
| 25 | Conditional process | `any(T=i32,T=i64)`, `not(T=u8)`, `all(…)` (combinators) |
| 26 | Fn dispatch | `T: Fn(i32)->i32`, `T: for<'a> Fn(&'a i32)->i32` (Fn bounds) |
| 27 | Multi-self transform | Same trait across TypeA, TypeB, Vec\<U\>, i32 |
| 28 | Static hash | `T: 'static` lifetime bound specialization |
| 29 | Dyn trait type | `T = &dyn Display`, `T = i32`, `T = String` (type patterns) |
| 30 | Combined bounds dedup | `T: Copy + Eq` multi-trait bound, `T = u8` bitmap |
| 31 | Many specs encode | 10 `#[when]` clauses (u8–f64) stress test |
| 32 | Tuple points | `T = (i32,i32)`, `(f64,f64)`, `(f32,f32)` squared-sum |
| 33 | Nested generics | `Vec<Vec<u8>>`, `Vec<String>`, `Vec<i32>` flatten |

## Approaches Compared

- **spec** — `spec!` macro: compile-time specialization dispatch (zero runtime cost)
- **generic** — Default generic trait implementation (no specialization)
- **enum** — Manual enum dispatch with match arms
- **runtime_typeid** — Runtime `TypeId::of` checks
- **dyn** — Dynamic dispatch via `&dyn Trait`
