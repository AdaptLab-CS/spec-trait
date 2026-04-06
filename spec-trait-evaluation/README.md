# spec-trait Evaluation

Benchmark suite that compares `spec!`/`#[when]` compile-time specialization against three alternative dispatch strategies:

| Approach | Description |
|---|---|
| **spec** | Compile-time specialization via `spec!`/`#[when]` macros |
| **naive** | Plain generic fallback (no specialization) |
| **enum** | Enum-based match dispatch |
| **runtime_typeid** | `TypeId`-based runtime branching |

## Examples

Each example implements the same operation with 4 dispatch approaches.
The `spec` variant exploits type-specific knowledge (e.g. memcpy, SIMD byte search, counting sort) that a naive generic fallback cannot use.

| # | Operation | `when` condition | spec strategy | Naive fallback |
|---|-----------|-----------------|---------------|----------------|
| ex01 | extend_from_slice | `T: Copy` | `ptr::copy_nonoverlapping` (memcpy) | clone loop |
| ex02 | to_bytes | `T: Copy` | raw `transmute` (zero-copy) | `format!()` per element |
| ex03 | contains | `T = u8` | `memchr` SIMD | `iter().any()` |
| ex04 | sort | `T = u8` | counting sort `[u32; 256]` | `sort_unstable()` |
| ex05 | histogram (max freq) | `T = u8` | `[u32; 256]` direct index | `HashMap<&T, u32>` |
| ex06 | dedup (keep first) | `T = u8` | `[bool; 256]` bitmap | `HashSet` |
| ex07 | hash_key | `K = u64` | identity XOR (no hashing) | `DefaultHasher` (SipHash) |
| ex08 | count_value | `T = u8` | `memchr_iter().count()` | `filter().count()` |

## Project structure

```
spec-trait-evaluation/
├── benches-crate/             # Rust crate with benchmarks
│   ├── Cargo.toml
│   ├── build.rs
│   ├── benches/
│   │   └── bench.rs           # Criterion harness (8 benchmark groups)
│   └── src/
│       ├── lib.rs
│       ├── data.rs            # Input generators (make_u8, make_i32, ...)
│       └── examples/          # 32 source files (8 examples × 4 variants)
├── scripts/
│   ├── run_all.sh             # Run benchmarks + collect all metrics
│   ├── collect_metrics.py     # Parse Criterion JSON → runtime.csv
│   ├── measure_build_time.sh  # Per-variant cargo check timing → build_time.csv
│   └── measure_binary_size.sh # Per-variant release rlib sizes → binary_size.csv
├── notebooks/
│   ├── eval.ipynb             # Analysis notebook (plots + summary tables)
│   └── requirements.txt       # Python dependencies
└── results/
    ├── runtime.csv
    ├── build_time.csv
    └── binary_size.csv
```

## Reproducing

### Prerequisites

- Rust nightly (edition 2024)
- Python 3 with `pandas`, `matplotlib`, `seaborn`

### Run individual measurements

```bash
# Runtime benchmarks (Criterion)
cargo bench --manifest-path benches-crate/Cargo.toml --bench bench
cargo bench --manifest-path benches-crate/Cargo.toml --bench bench -- --warm-up-time 1 --measurement-time 3 --sample-size 10
cargo bench --manifest-path benches-crate/Cargo.toml --bench bench -- --warm-up-time 1 --measurement-time 3 --sample-size 10 <name_filter>

# Collect runtime CSV from Criterion output
python3 scripts/collect_metrics.py \
  --criterion-dir benches-crate/target/criterion \
  --output results/runtime.csv

# Compile time (3 iterations, 8 examples)
bash scripts/measure_build_time.sh 3 8

# Binary size
bash scripts/measure_binary_size.sh 8
```

### Notebook

```bash
pip install -r notebooks/requirements.txt
jupyter notebook notebooks/eval_pretty.ipynb
```

## Benchmark parameters

- **Input sizes**: 256, 1024, 4096, 16384, 65536 elements
- **Data type**: `u8` (most examples), `u64` (ex07 hash_key)
- **Criterion config**: warm-up 3s, measurement 5s, 100 samples (default); `--quick` uses 1s/3s/10
- **Build time**: `cargo check` from clean local crate (dependencies pre-cached), 3 iterations
- **Binary size**: release-mode `rlib` artifact (`opt-level=3`, `lto=false`, `codegen-units=16`)
