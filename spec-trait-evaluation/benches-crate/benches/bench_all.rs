use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use spec_trait_evaluation::data::*;
use spec_trait_evaluation::examples::*;

const SIZES: [usize; 5] = [256, 1024, 4096, 16384, 65536];

// ── Ex01: extend_from_slice — Copy memcpy vs Clone loop ────────────────
fn bench_ex01(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex01_extend");
    for &n in &SIZES {
        let src = make_u8(n);
        g.throughput(Throughput::Elements(n as u64));
        let ext = ex01_spec::Extender;
        let ext_naive = ex01_naive::Extender;
        let ext_rt = ex01_runtime_typeid::Extender;
        g.bench_with_input(BenchmarkId::new("spec", n), &src, |b, src| {
            b.iter(|| {
                let mut dst = Vec::new();
                ex01_spec::extend_u8(&ext, &mut dst, std::hint::black_box(src));
                dst
            })
        });
        g.bench_with_input(BenchmarkId::new("naive", n), &src, |b, src| {
            b.iter(|| {
                let mut dst = Vec::new();
                ex01_naive::extend_naive(&ext_naive, &mut dst, std::hint::black_box(src));
                dst
            })
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &src, |b, src| {
            b.iter(|| {
                let mut dst = Vec::new();
                let e = ex01_enum::ExtendEnum::U8(src);
                ex01_enum::extend_enum(&e, &mut dst);
                dst
            })
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &src, |b, src| {
            b.iter(|| {
                let mut dst = Vec::new();
                ex01_runtime_typeid::extend_runtime(&ext_rt, &mut dst, std::hint::black_box(src));
                dst
            })
        });
    }
    g.finish();
}

// ── Ex02: to_bytes — Copy raw transmute vs Display formatting ──────────
fn bench_ex02(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex02_to_bytes");
    for &n in &SIZES {
        let data = make_u8(n);
        g.throughput(Throughput::Elements(n as u64));
        let s = ex02_spec::Serializer;
        let s_naive = ex02_naive::Serializer;
        let s_rt = ex02_runtime_typeid::Serializer;
        g.bench_with_input(BenchmarkId::new("spec", n), &data, |b, data| {
            b.iter(|| ex02_spec::to_bytes_u8(&s, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("naive", n), &data, |b, data| {
            b.iter(|| ex02_naive::to_bytes_naive(&s_naive, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &data, |b, data| {
            let e = ex02_enum::ToBytesEnum::U8(data);
            b.iter(|| ex02_enum::to_bytes_enum(&e))
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &data, |b, data| {
            b.iter(|| ex02_runtime_typeid::to_bytes_runtime(&s_rt, std::hint::black_box(data)))
        });
    }
    g.finish();
}

// ── Ex03: contains — memchr SIMD vs linear scan ────────────────────────
fn bench_ex03(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex03_contains");
    for &n in &SIZES {
        let data = make_u8(n);
        let needle: u8 = 200;
        g.throughput(Throughput::Elements(n as u64));
        let sc = ex03_spec::Searcher;
        let sc_naive = ex03_naive::Searcher;
        let sc_rt = ex03_runtime_typeid::Searcher;
        g.bench_with_input(BenchmarkId::new("spec", n), &data, |b, data| {
            b.iter(|| ex03_spec::contains_u8(&sc, std::hint::black_box(data), &needle))
        });
        g.bench_with_input(BenchmarkId::new("naive", n), &data, |b, data| {
            b.iter(|| ex03_naive::contains_naive(&sc_naive, std::hint::black_box(data), &needle))
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &data, |b, data| {
            let e = ex03_enum::ContainsEnum::U8(data, needle);
            b.iter(|| ex03_enum::contains_enum(&e))
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &data, |b, data| {
            b.iter(|| ex03_runtime_typeid::contains_runtime(&sc_rt, std::hint::black_box(data), &needle))
        });
    }
    g.finish();
}

// ── Ex04: sort — counting sort (u8) vs sort_unstable ───────────────────
fn bench_ex04(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex04_sort");
    for &n in &SIZES {
        let base = make_u8(n);
        g.throughput(Throughput::Elements(n as u64));
        let sr = ex04_spec::Sorter;
        let sr_naive = ex04_naive::Sorter;
        let sr_rt = ex04_runtime_typeid::Sorter;
        g.bench_with_input(BenchmarkId::new("spec", n), &base, |b, base| {
            b.iter(|| {
                let mut d = base.clone();
                ex04_spec::sort_u8(&sr, std::hint::black_box(&mut d));
                d
            })
        });
        g.bench_with_input(BenchmarkId::new("naive", n), &base, |b, base| {
            b.iter(|| {
                let mut d = base.clone();
                ex04_naive::sort_naive(&sr_naive, std::hint::black_box(&mut d));
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
                ex04_runtime_typeid::sort_runtime(&sr_rt, std::hint::black_box(&mut d));
                d
            })
        });
    }
    g.finish();
}

// ── Ex05: histogram — array[256] vs HashMap ────────────────────────────
fn bench_ex05(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex05_histogram");
    for &n in &SIZES {
        let data = make_u8(n);
        g.throughput(Throughput::Elements(n as u64));
        let h = ex05_spec::HistogramOp;
        let h_naive = ex05_naive::HistogramOp;
        let h_rt = ex05_runtime_typeid::HistogramOp;
        g.bench_with_input(BenchmarkId::new("spec", n), &data, |b, data| {
            b.iter(|| ex05_spec::max_freq_u8(&h, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("naive", n), &data, |b, data| {
            b.iter(|| ex05_naive::max_freq_naive(&h_naive, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &data, |b, data| {
            let e = ex05_enum::HistEnum::U8(data);
            b.iter(|| ex05_enum::histogram_enum(&e))
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &data, |b, data| {
            b.iter(|| ex05_runtime_typeid::max_freq_runtime(&h_rt, std::hint::black_box(data)))
        });
    }
    g.finish();
}

// ── Ex06: dedup — bitmap[256] vs HashSet ────────────────────────────────
fn bench_ex06(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex06_dedup");
    for &n in &SIZES {
        let data = make_u8(n);
        g.throughput(Throughput::Elements(n as u64));
        let dd = ex06_spec::Deduplicator;
        let dd_naive = ex06_naive::Deduplicator;
        let dd_rt = ex06_runtime_typeid::Deduplicator;
        g.bench_with_input(BenchmarkId::new("spec", n), &data, |b, data| {
            b.iter(|| ex06_spec::dedup_u8(&dd, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("naive", n), &data, |b, data| {
            b.iter(|| ex06_naive::dedup_naive(&dd_naive, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &data, |b, data| {
            let e = ex06_enum::DedupEnum::U8(data);
            b.iter(|| ex06_enum::dedup_enum(&e))
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &data, |b, data| {
            b.iter(|| ex06_runtime_typeid::dedup_runtime(&dd_rt, std::hint::black_box(data)))
        });
    }
    g.finish();
}

// ── Ex07: hash_key — identity hash (u64) vs SipHash ────────────────────
fn bench_ex07(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex07_hash_key");
    for &n in &SIZES {
        let keys = make_u64(n);
        g.throughput(Throughput::Elements(n as u64));
        let h = ex07_spec::HashOp;
        let h_naive = ex07_naive::HashOp;
        let h_rt = ex07_runtime_typeid::HashOp;
        g.bench_with_input(BenchmarkId::new("spec", n), &keys, |b, keys| {
            b.iter(|| ex07_spec::hash_all_u64(&h, std::hint::black_box(keys)))
        });
        g.bench_with_input(BenchmarkId::new("naive", n), &keys, |b, keys| {
            b.iter(|| ex07_naive::hash_all_naive(&h_naive, std::hint::black_box(keys)))
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &keys, |b, keys| {
            let e = ex07_enum::HashEnum::U64(keys);
            b.iter(|| ex07_enum::hash_all_enum(&e))
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &keys, |b, keys| {
            b.iter(|| ex07_runtime_typeid::hash_all_runtime(&h_rt, std::hint::black_box(keys)))
        });
    }
    g.finish();
}

// ── Ex08: count_value — memchr SIMD vs filter().count() ────────────────
fn bench_ex08(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex08_count_value");
    for &n in &SIZES {
        let data = make_u8(n);
        let needle: u8 = 42;
        g.throughput(Throughput::Elements(n as u64));
        let op = ex08_spec::CountValOp;
        let op_naive = ex08_naive::CountValOp;
        let op_rt = ex08_runtime_typeid::CountValOp;
        g.bench_with_input(BenchmarkId::new("spec", n), &data, |b, data| {
            b.iter(|| ex08_spec::count_val_u8(&op, std::hint::black_box(data), &needle))
        });
        g.bench_with_input(BenchmarkId::new("naive", n), &data, |b, data| {
            b.iter(|| ex08_naive::count_val_naive(&op_naive, std::hint::black_box(data), &needle))
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &data, |b, data| {
            let e = ex08_enum::CountValEnum::U8(data, needle);
            b.iter(|| ex08_enum::count_val_enum(&e))
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &data, |b, data| {
            b.iter(|| ex08_runtime_typeid::count_val_runtime(&op_rt, std::hint::black_box(data), &needle))
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
