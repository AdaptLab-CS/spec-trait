# spec-trait Evaluation

Benchmark suite that compares `spec!`/`#[when]` compile-time specialization against four alternative dispatch strategies:

| Approach | Description |
|---|---|
| **spec** | Compile-time specialization via `spec!`/`#[when]` macros |
| **dyn** | Trait-object dynamic dispatch |
| **enum** | Enum-based match dispatch |
| **generic** | Plain generic (no specialization) |
| **runtime_typeid** | `TypeId`-based runtime branching |

## Examples

Each example implements the same operation with up to 5 dispatch approaches.
The `spec` variant exploits type-specific knowledge (e.g. fixed-size arrays, SIMD byte search) that a generic fallback cannot use.

| # | Operation | spec strategy | Generic fallback | Speedup (n=65536) |
|---|-----------|---------------|------------------|-------------------|
| ex01 | Histogram (max frequency) | `[u32; 256]` direct index | `HashMap<u8, u32>` | 22–66× |
| ex02 | Set intersection count | `[bool; 256]` bitmap | `HashSet<u8>` | 36–41× |
| ex03 | Contains search | `memchr` SIMD | `iter().any()` | 19–20× |
| ex04 | Sort | `[u32; 256]` counting sort | `sort_unstable()` | 7–11× |
| ex05 | Distinct count | `[bool; 256]` bitmap | `HashSet<u8>` | 43–46× |
| ex06 | Bulk serialize | `to_vec()` zero-copy | `format!()` per element | 166–1993× |
| ex07 | Dedup (keep first) | `[bool; 256]` bitmap | `HashSet<u8>` | 21–24× |
| ex08 | Count value occurrences | `memchr_iter().count()` | `filter().count()` | 4–7× |

## Project structure

```
spec-trait-evaluation/
├── benches-crate/           # Rust crate with benchmarks
│   ├── Cargo.toml
│   ├── build.rs
│   ├── benches/
│   │   └── bench_all.rs     # Criterion harness (8 benchmark groups)
│   └── src/
│       ├── lib.rs
│       ├── data.rs           # Input generators (make_u8, make_i32, ...)
│       └── examples/         # 36 source files (8 examples × 4–5 variants)
├── scripts/
│   ├── run_all.sh            # Run benchmarks + collect all metrics
│   ├── collect_metrics.py    # Parse Criterion JSON → runtime.csv
│   ├── measure_build_time.sh # Per-variant cargo check timing → build_time.csv
│   └── measure_binary_size.sh# Per-variant release rlib sizes → binary_size.csv
├── notebooks/
│   ├── eval.ipynb            # Analysis notebook (plots + summary tables)
│   └── requirements.txt      # Python dependencies
└── results/
    ├── runtime.csv
    ├── build_time.csv
    └── binary_size.csv
```

## Reproducing

### Prerequisites

- Rust nightly (edition 2024)
- Python 3 with `pandas`, `matplotlib`, `seaborn`

### Run everything

```bash
cd scripts
bash run_all.sh          # full benchmarks (slow)
bash run_all.sh --quick  # quick benchmarks (fewer samples)
```

### Run individual measurements

```bash
# Runtime benchmarks (Criterion)
cargo bench --manifest-path benches-crate/Cargo.toml --bench bench_all

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
jupyter notebook notebooks/eval.ipynb
```

## Benchmark parameters

- **Input sizes**: 256, 1024, 4096, 16384, 65536 elements
- **Data type**: `u8` (all examples use byte-level specialization)
- **Criterion config**: warm-up 3s, measurement 5s, 100 samples (default); `--quick` uses 1s/3s/10
- **Build time**: `cargo check` from clean local crate (dependencies pre-cached), 3 iterations
- **Binary size**: release-mode `rlib` artifact (`opt-level=3`, `lto=false`, `codegen-units=16`)
