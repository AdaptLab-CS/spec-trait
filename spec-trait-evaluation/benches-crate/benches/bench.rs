use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use spec_trait_evaluation::data::*;
use spec_trait_evaluation::examples::*;

/// Data sizes: tiny (dispatch-overhead-dominated) → medium (work-dominated).
/// At size 8 the dispatch cost is a large fraction of per-call time;
/// at size 2048 the actual work dominates and dispatch is negligible.
const SIZES: [usize; 9] = [8, 16, 32, 64, 128, 256, 512, 1024, 2048]; //, 4096, 8192, 16384, 32768, 65536, 131072];

// ── Ex01: extend — Copy memcpy vs Clone loop ──────────────────────────
fn bench_ex01(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex01_extend");
    for &size in &SIZES {
        let src = make_u8(size);
        let ext = ex01_spec::Extender;
        let ext_naive = ex01_naive::Extender;
        let ext_rt = ex01_runtime_typeid::Extender;
        g.bench_with_input(BenchmarkId::new("spec", size), &src, |b, src| {
            let mut dst = Vec::with_capacity(size);
            b.iter(|| {
                dst.clear();
                ex01_spec::extend_u8(&ext, &mut dst, std::hint::black_box(src));
                std::hint::black_box(&dst);
            })
        });
        g.bench_with_input(BenchmarkId::new("naive", size), &src, |b, src| {
            let mut dst = Vec::with_capacity(size);
            b.iter(|| {
                dst.clear();
                ex01_naive::extend_naive(&ext_naive, &mut dst, std::hint::black_box(src));
                std::hint::black_box(&dst);
            })
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", size), &src, |b, src| {
            let mut dst = Vec::with_capacity(size);
            b.iter(|| {
                dst.clear();
                ex01_runtime_typeid::extend_runtime(&ext_rt, &mut dst, std::hint::black_box(src));
                std::hint::black_box(&dst);
            })
        });
    }
    g.finish();
}

// ── Ex02: to_bytes — raw transmute vs Display formatting ───────────────
fn bench_ex02(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex02_to_bytes");
    for &size in &SIZES {
        let data = make_u8(size);
        let s = ex02_spec::Serializer;
        let s_naive = ex02_naive::Serializer;
        let s_rt = ex02_runtime_typeid::Serializer;
        g.bench_with_input(BenchmarkId::new("spec", size), &data, |b, data| {
            b.iter(|| {
                std::hint::black_box(ex02_spec::to_bytes_u8(&s, std::hint::black_box(data)))
            })
        });
        g.bench_with_input(BenchmarkId::new("naive", size), &data, |b, data| {
            b.iter(|| {
                std::hint::black_box(ex02_naive::to_bytes_naive(&s_naive, std::hint::black_box(data)))
            })
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", size), &data, |b, data| {
            b.iter(|| {
                std::hint::black_box(ex02_runtime_typeid::to_bytes_runtime(&s_rt, std::hint::black_box(data)))
            })
        });
    }
    g.finish();
}

// ── Ex03: contains — memchr SIMD vs linear scan ────────────────────────
fn bench_ex03(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex03_contains");
    let needle: u8 = 200;
    for &size in &SIZES {
        let data = make_u8(size);
        let sc = ex03_spec::Searcher;
        let sc_naive = ex03_naive::Searcher;
        let sc_rt = ex03_runtime_typeid::Searcher;
        g.bench_with_input(BenchmarkId::new("spec", size), &data, |b, data| {
            b.iter(|| {
                std::hint::black_box(ex03_spec::contains_u8(&sc, std::hint::black_box(data), &needle))
            })
        });
        g.bench_with_input(BenchmarkId::new("naive", size), &data, |b, data| {
            b.iter(|| {
                std::hint::black_box(ex03_naive::contains_naive(&sc_naive, std::hint::black_box(data), &needle))
            })
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", size), &data, |b, data| {
            b.iter(|| {
                std::hint::black_box(ex03_runtime_typeid::contains_runtime(&sc_rt, std::hint::black_box(data), &needle))
            })
        });
    }
    g.finish();
}

// ── Ex04: sort — counting sort O(N) vs sort_unstable O(N log N) ───────
fn bench_ex04(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex04_sort");
    for &size in &SIZES {
        let base = make_u8(size);
        let sr = ex04_spec::Sorter;
        let sr_naive = ex04_naive::Sorter;
        let sr_rt = ex04_runtime_typeid::Sorter;
        g.bench_with_input(BenchmarkId::new("spec", size), &base, |b, base| {
            b.iter(|| {
                let mut d = base.clone();
                ex04_spec::sort_u8(&sr, std::hint::black_box(&mut d));
                std::hint::black_box(&d);
            })
        });
        g.bench_with_input(BenchmarkId::new("naive", size), &base, |b, base| {
            b.iter(|| {
                let mut d = base.clone();
                ex04_naive::sort_naive(&sr_naive, std::hint::black_box(&mut d));
                std::hint::black_box(&d);
            })
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", size), &base, |b, base| {
            b.iter(|| {
                let mut d = base.clone();
                ex04_runtime_typeid::sort_runtime(&sr_rt, std::hint::black_box(&mut d));
                std::hint::black_box(&d);
            })
        });
    }
    g.finish();
}

// ── Ex05: histogram — array[256] vs HashMap ────────────────────────────
fn bench_ex05(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex05_histogram");
    for &size in &SIZES {
        let data = make_u8(size);
        let h = ex05_spec::HistogramOp;
        let h_naive = ex05_naive::HistogramOp;
        let h_rt = ex05_runtime_typeid::HistogramOp;
        g.bench_with_input(BenchmarkId::new("spec", size), &data, |b, data| {
            b.iter(|| {
                std::hint::black_box(ex05_spec::max_freq_u8(&h, std::hint::black_box(data)))
            })
        });
        g.bench_with_input(BenchmarkId::new("naive", size), &data, |b, data| {
            b.iter(|| {
                std::hint::black_box(ex05_naive::max_freq_naive(&h_naive, std::hint::black_box(data)))
            })
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", size), &data, |b, data| {
            b.iter(|| {
                std::hint::black_box(ex05_runtime_typeid::max_freq_runtime(&h_rt, std::hint::black_box(data)))
            })
        });
    }
    g.finish();
}

// ── Ex06: dedup — bitmap[256] vs HashSet ───────────────────────────────
fn bench_ex06(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex06_dedup");
    for &size in &SIZES {
        let data = make_u8(size);
        let dd = ex06_spec::Deduplicator;
        let dd_naive = ex06_naive::Deduplicator;
        let dd_rt = ex06_runtime_typeid::Deduplicator;
        g.bench_with_input(BenchmarkId::new("spec", size), &data, |b, data| {
            b.iter(|| {
                std::hint::black_box(ex06_spec::dedup_u8(&dd, std::hint::black_box(data)))
            })
        });
        g.bench_with_input(BenchmarkId::new("naive", size), &data, |b, data| {
            b.iter(|| {
                std::hint::black_box(ex06_naive::dedup_naive(&dd_naive, std::hint::black_box(data)))
            })
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", size), &data, |b, data| {
            b.iter(|| {
                std::hint::black_box(ex06_runtime_typeid::dedup_runtime(&dd_rt, std::hint::black_box(data)))
            })
        });
    }
    g.finish();
}

// ── Ex07: hash_key — identity XOR vs SipHash ───────────────────────────
fn bench_ex07(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex07_hash_key");
    for &size in &SIZES {
        let keys = make_u64(size);
        let h = ex07_spec::HashOp;
        let h_naive = ex07_naive::HashOp;
        let h_rt = ex07_runtime_typeid::HashOp;
        g.bench_with_input(BenchmarkId::new("spec", size), &keys, |b, keys| {
            b.iter(|| {
                std::hint::black_box(ex07_spec::hash_all_u64(&h, std::hint::black_box(keys)))
            })
        });
        g.bench_with_input(BenchmarkId::new("naive", size), &keys, |b, keys| {
            b.iter(|| {
                std::hint::black_box(ex07_naive::hash_all_naive(&h_naive, std::hint::black_box(keys)))
            })
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", size), &keys, |b, keys| {
            b.iter(|| {
                std::hint::black_box(ex07_runtime_typeid::hash_all_runtime(&h_rt, std::hint::black_box(keys)))
            })
        });
    }
    g.finish();
}

// ── Ex08: count_value — memchr SIMD vs filter().count() ────────────────
fn bench_ex08(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex08_count_value");
    let needle: u8 = 42;
    for &size in &SIZES {
        let data = make_u8(size);
        let op = ex08_spec::CountValOp;
        let op_naive = ex08_naive::CountValOp;
        let op_rt = ex08_runtime_typeid::CountValOp;
        g.bench_with_input(BenchmarkId::new("spec", size), &data, |b, data| {
            b.iter(|| {
                std::hint::black_box(ex08_spec::count_val_u8(&op, std::hint::black_box(data), &needle))
            })
        });
        g.bench_with_input(BenchmarkId::new("naive", size), &data, |b, data| {
            b.iter(|| {
                std::hint::black_box(ex08_naive::count_val_naive(&op_naive, std::hint::black_box(data), &needle))
            })
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", size), &data, |b, data| {
            b.iter(|| {
                std::hint::black_box(ex08_runtime_typeid::count_val_runtime(&op_rt, std::hint::black_box(data), &needle))
            })
        });
    }
    g.finish();
}

criterion_group!(
    benches,
    bench_ex01,
    bench_ex02,
    bench_ex03,
    bench_ex04,
    bench_ex05,
    bench_ex06,
    bench_ex07,
    bench_ex08,
);
criterion_main!(benches);
