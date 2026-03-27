use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use spec_trait_evaluation::data::*;
use spec_trait_evaluation::examples::*;

const SIZES: [usize; 5] = [256, 1024, 4096, 16384, 65536];

// ── Ex01: Histogram — max frequency count (u8) ─────────────────────────
fn bench_ex01(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex01_histogram_u8");
    for &n in &SIZES {
        let data = make_u8(n);
        g.throughput(Throughput::Elements(n as u64));
        let h = ex01_spec::HistogramOp;
        g.bench_with_input(BenchmarkId::new("spec", n), &data, |b, data| {
            b.iter(|| ex01_spec::max_freq_u8(&h, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("dyn", n), &data, |b, data| {
            let d = ex01_dyn::DynHistU8 { data };
            b.iter(|| ex01_dyn::max_freq_dyn(&d))
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &data, |b, data| {
            let e = ex01_enum::HistEnum::U8(data);
            b.iter(|| ex01_enum::max_freq_enum(&e))
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &data, |b, data| {
            b.iter(|| ex01_runtime_typeid::max_freq_runtime_u8(std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("generic", n), &data, |b, data| {
            b.iter(|| ex01_generic::max_freq_generic_u8(std::hint::black_box(data)))
        });
    }
    g.finish();
}

// ── Ex02: Set intersection count (u8) ───────────────────────────────────
fn bench_ex02(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex02_intersect_u8");
    for &n in &SIZES {
        let a = make_u8(n);
        let b_data = make_u8(n);
        g.throughput(Throughput::Elements(n as u64));
        let op = ex02_spec::IntersectOp;
        g.bench_with_input(BenchmarkId::new("spec", n), &n, |bench, _| {
            bench.iter(|| ex02_spec::intersect_u8(&op, std::hint::black_box(&a), std::hint::black_box(&b_data)))
        });
        g.bench_with_input(BenchmarkId::new("dyn", n), &n, |bench, _| {
            let d = ex02_dyn::DynIntersectU8 { a: &a, b: &b_data };
            bench.iter(|| ex02_dyn::intersect_dyn(&d))
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &n, |bench, _| {
            let e = ex02_enum::IntersectEnum::U8(&a, &b_data);
            bench.iter(|| ex02_enum::intersect_enum(&e))
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &n, |bench, _| {
            bench.iter(|| ex02_runtime_typeid::intersect_runtime_u8(std::hint::black_box(&a), std::hint::black_box(&b_data)))
        });
        g.bench_with_input(BenchmarkId::new("generic", n), &n, |bench, _| {
            bench.iter(|| ex02_generic::intersect_generic_u8(std::hint::black_box(&a), std::hint::black_box(&b_data)))
        });
    }
    g.finish();
}

// ── Ex03: Contains / memchr (u8) ────────────────────────────────────────
fn bench_ex03(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex03_contains_u8");
    for &n in &SIZES {
        let data = make_u8(n);
        let needle: u8 = 200;
        g.throughput(Throughput::Elements(n as u64));
        let sc = ex03_spec::Searcher;
        g.bench_with_input(BenchmarkId::new("spec", n), &data, |b, data| {
            b.iter(|| ex03_spec::contains_u8(&sc, std::hint::black_box(data), &needle))
        });
        g.bench_with_input(BenchmarkId::new("generic", n), &data, |b, data| {
            b.iter(|| ex03_generic::contains_generic_u8(std::hint::black_box(data), &needle))
        });
        g.bench_with_input(BenchmarkId::new("dyn", n), &data, |b, data| {
            let s = ex03_dyn::DynU8Search { haystack: data, needle };
            b.iter(|| ex03_dyn::contains_dyn(&s))
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &data, |b, data| {
            let e = ex03_enum::ContainsEnum::U8(data, needle);
            b.iter(|| ex03_enum::contains_enum(&e))
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &data, |b, data| {
            b.iter(|| ex03_runtime_typeid::contains_runtime_u8(std::hint::black_box(data), &needle))
        });
    }
    g.finish();
}

// ── Ex04: Sort (u8 counting sort) ───────────────────────────────────────
fn bench_ex04(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex04_sort_u8");
    for &n in &SIZES {
        let base = make_u8(n);
        g.throughput(Throughput::Elements(n as u64));
        let sr = ex04_spec::Sorter;
        g.bench_with_input(BenchmarkId::new("spec", n), &base, |b, base| {
            b.iter(|| {
                let mut d = base.clone();
                ex04_spec::sort_u8(&sr, std::hint::black_box(&mut d));
                d
            })
        });
        g.bench_with_input(BenchmarkId::new("generic", n), &base, |b, base| {
            b.iter(|| {
                let mut d = base.clone();
                ex04_generic::sort_generic_u8(std::hint::black_box(&mut d));
                d
            })
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &base, |b, base| {
            b.iter(|| {
                let mut d = base.clone();
                ex04_enum::sort_enum(&mut ex04_enum::SortEnum::U8(&mut d));
                d
            })
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &base, |b, base| {
            b.iter(|| {
                let mut d = base.clone();
                ex04_runtime_typeid::sort_runtime_u8(std::hint::black_box(&mut d));
                d
            })
        });
    }
    g.finish();
}

// ── Ex05: Distinct count (u8) ───────────────────────────────────────────
fn bench_ex05(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex05_distinct_u8");
    for &n in &SIZES {
        let data = make_u8(n);
        g.throughput(Throughput::Elements(n as u64));
        let op = ex05_spec::DistinctOp;
        g.bench_with_input(BenchmarkId::new("spec", n), &data, |b, data| {
            b.iter(|| ex05_spec::distinct_u8(&op, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("dyn", n), &data, |b, data| {
            let d = ex05_dyn::DynDistinctU8 { data };
            b.iter(|| ex05_dyn::distinct_dyn(&d))
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &data, |b, data| {
            let e = ex05_enum::DistinctEnum::U8(data);
            b.iter(|| ex05_enum::distinct_enum(&e))
        });
        g.bench_with_input(BenchmarkId::new("generic", n), &data, |b, data| {
            b.iter(|| ex05_generic::distinct_generic_u8(std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &data, |b, data| {
            b.iter(|| ex05_runtime_typeid::distinct_runtime_u8(std::hint::black_box(data)))
        });
    }
    g.finish();
}

// ── Ex06: Bulk serialize (u8 zero-copy) ─────────────────────────────────
fn bench_ex06(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex06_bulkser_u8");
    for &n in &SIZES {
        let data = make_u8(n);
        g.throughput(Throughput::Elements(n as u64));
        let bs = ex06_spec::BulkSerializer;
        g.bench_with_input(BenchmarkId::new("spec", n), &data, |b, data| {
            b.iter(|| ex06_spec::ser_bulk_u8(&bs, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("generic", n), &data, |b, data| {
            b.iter(|| ex06_generic::ser_bulk_generic_u8(std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &data, |b, data| {
            let e = ex06_enum::BulkSerEnum::U8(data);
            b.iter(|| ex06_enum::bulkser_enum(&e))
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &data, |b, data| {
            b.iter(|| ex06_runtime_typeid::ser_bulk_runtime_u8(std::hint::black_box(data)))
        });
    }
    g.finish();
}

// ── Ex07: Dedup — full dedup (u8 bitmap) ────────────────────────────────
fn bench_ex07(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex07_dedup_u8");
    for &n in &SIZES {
        let data = make_u8(n);
        g.throughput(Throughput::Elements(n as u64));
        let dd = ex07_spec::Deduplicator;
        g.bench_with_input(BenchmarkId::new("spec", n), &data, |b, data| {
            b.iter(|| ex07_spec::dedup_u8(&dd, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("generic", n), &data, |b, data| {
            b.iter(|| ex07_generic::dedup_generic_u8(std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &data, |b, data| {
            let e = ex07_enum::DedupEnum::U8(data);
            b.iter(|| ex07_enum::dedup_enum(&e))
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &data, |b, data| {
            b.iter(|| ex07_runtime_typeid::dedup_runtime_u8(std::hint::black_box(data)))
        });
    }
    g.finish();
}

// ── Ex08: Count value occurrences (u8 memchr) ──────────────────────────
fn bench_ex08(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex08_countval_u8");
    for &n in &SIZES {
        let data = make_u8(n);
        let needle: u8 = 42;
        g.throughput(Throughput::Elements(n as u64));
        let op = ex08_spec::CountValOp;
        g.bench_with_input(BenchmarkId::new("spec", n), &data, |b, data| {
            b.iter(|| ex08_spec::count_val_u8(&op, std::hint::black_box(data), &needle))
        });
        g.bench_with_input(BenchmarkId::new("generic", n), &data, |b, data| {
            b.iter(|| ex08_generic::count_val_generic_u8(std::hint::black_box(data), needle))
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &data, |b, data| {
            let e = ex08_enum::CountValEnum::U8(data, needle);
            b.iter(|| ex08_enum::count_val_enum(&e))
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &data, |b, data| {
            b.iter(|| ex08_runtime_typeid::count_val_runtime_u8(std::hint::black_box(data), needle))
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
