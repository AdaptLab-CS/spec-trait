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

// ── Ex09: format_collect — 'static lifetime bound ──────────────────────
fn bench_ex09(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex09_format_collect");
    for &n in &SIZES {
        let data = make_i32(n);
        g.throughput(Throughput::Elements(n as u64));
        let f = ex09_spec::Formatter;
        let f_naive = ex09_naive::Formatter;
        let f_enum = ex09_enum::Formatter;
        let f_rt = ex09_runtime_typeid::Formatter;
        g.bench_with_input(BenchmarkId::new("spec", n), &data, |b, data| {
            b.iter(|| ex09_spec::format_collect_i32(&f, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("naive", n), &data, |b, data| {
            b.iter(|| ex09_naive::format_collect_naive(&f_naive, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &data, |b, data| {
            b.iter(|| ex09_enum::format_collect_enum(&f_enum, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &data, |b, data| {
            b.iter(|| ex09_runtime_typeid::format_collect_runtime(&f_rt, std::hint::black_box(data)))
        });
    }
    g.finish();
}

// ── Ex10: sum_slice — reference lifetime specialization ────────────────
fn bench_ex10(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex10_sum_slice");
    for &n in &SIZES {
        let refs = make_static_i32_refs(n);
        g.throughput(Throughput::Elements(n as u64));
        let s = ex10_spec::Summer;
        let s_naive = ex10_naive::Summer;
        let s_enum = ex10_enum::Summer;
        let s_rt = ex10_runtime_typeid::Summer;
        g.bench_with_input(BenchmarkId::new("spec", n), &refs, |b, refs| {
            b.iter(|| ex10_spec::sum_slice_static_refs(&s, std::hint::black_box(refs)))
        });
        g.bench_with_input(BenchmarkId::new("naive", n), &refs, |b, refs| {
            b.iter(|| ex10_naive::sum_slice_naive(&s_naive, std::hint::black_box(refs)))
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &refs, |b, refs| {
            b.iter(|| ex10_enum::sum_slice_enum(&s_enum, std::hint::black_box(refs)))
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &refs, |b, refs| {
            b.iter(|| ex10_runtime_typeid::sum_slice_runtime(&s_rt, std::hint::black_box(refs)))
        });
    }
    g.finish();
}

// ── Ex11: map_transform — higher-order fn pointer ──────────────────────
fn bench_ex11(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex11_map_transform");
    for &n in &SIZES {
        let data = make_i32(n);
        g.throughput(Throughput::Elements(n as u64));
        let m = ex11_spec::Mapper;
        let m_naive = ex11_naive::Mapper;
        let m_enum = ex11_enum::Mapper;
        let m_rt = ex11_runtime_typeid::Mapper;
        g.bench_with_input(BenchmarkId::new("spec", n), &data, |b, data| {
            b.iter(|| ex11_spec::map_transform_fn_ptr(&m, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("naive", n), &data, |b, data| {
            b.iter(|| ex11_naive::map_transform_naive(&m_naive, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &data, |b, data| {
            b.iter(|| ex11_enum::map_transform_enum(&m_enum, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &data, |b, data| {
            b.iter(|| ex11_runtime_typeid::map_transform_runtime(&m_rt, std::hint::black_box(data)))
        });
    }
    g.finish();
}

// ── Ex12: fold_with — higher-ranked trait bound ────────────────────────
fn bench_ex12(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex12_fold_with");
    for &n in &SIZES {
        let data = make_i32(n);
        g.throughput(Throughput::Elements(n as u64));
        let folder = ex12_spec::Folder;
        let folder_naive = ex12_naive::Folder;
        let folder_enum = ex12_enum::Folder;
        let folder_rt = ex12_runtime_typeid::Folder;
        g.bench_with_input(BenchmarkId::new("spec", n), &data, |b, data| {
            b.iter(|| ex12_spec::fold_with_hrtb(&folder, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("naive", n), &data, |b, data| {
            b.iter(|| ex12_naive::fold_with_naive(&folder_naive, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &data, |b, data| {
            b.iter(|| ex12_enum::fold_with_enum(&folder_enum, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &data, |b, data| {
            b.iter(|| ex12_runtime_typeid::fold_with_runtime(&folder_rt, std::hint::black_box(data)))
        });
    }
    g.finish();
}

// ── Ex13: flatten_sum — compound all() condition ───────────────────────
fn bench_ex13(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex13_flatten_sum");
    for &n in &SIZES {
        let outer = make_nested_vec_u8(n / 16, 16);
        g.throughput(Throughput::Elements(n as u64));
        let fl = ex13_spec::Flattener;
        let fl_naive = ex13_naive::Flattener;
        let fl_enum = ex13_enum::Flattener;
        let fl_rt = ex13_runtime_typeid::Flattener;
        g.bench_with_input(BenchmarkId::new("spec", n), &outer, |b, outer| {
            b.iter(|| ex13_spec::flatten_sum_vec_u8(&fl, std::hint::black_box(outer)))
        });
        g.bench_with_input(BenchmarkId::new("naive", n), &outer, |b, outer| {
            b.iter(|| ex13_naive::flatten_sum_naive(&fl_naive, std::hint::black_box(outer)))
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &outer, |b, outer| {
            b.iter(|| ex13_enum::flatten_sum_enum(&fl_enum, std::hint::black_box(outer)))
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &outer, |b, outer| {
            b.iter(|| ex13_runtime_typeid::flatten_sum_runtime(&fl_rt, std::hint::black_box(outer)))
        });
    }
    g.finish();
}

// ── Ex14: encode_val — negation/alternation dispatch ───────────────────
fn bench_ex14(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex14_encode_val");
    for &n in &SIZES {
        let data = make_i32(n);
        g.throughput(Throughput::Elements(n as u64));
        let enc = ex14_spec::Encoder;
        let enc_naive = ex14_naive::Encoder;
        let enc_enum = ex14_enum::Encoder;
        let enc_rt = ex14_runtime_typeid::Encoder;
        g.bench_with_input(BenchmarkId::new("spec", n), &data, |b, data| {
            b.iter(|| ex14_spec::encode_val_i32(&enc, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("naive", n), &data, |b, data| {
            b.iter(|| ex14_naive::encode_val_naive(&enc_naive, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &data, |b, data| {
            b.iter(|| ex14_enum::encode_val_enum(&enc_enum, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &data, |b, data| {
            b.iter(|| ex14_runtime_typeid::encode_val_runtime(&enc_rt, std::hint::black_box(data)))
        });
    }
    g.finish();
}

// ── Ex15: compute_sum — trait object specialization ────────────────────
fn bench_ex15(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex15_compute_sum");
    for &n in &SIZES {
        let values: Vec<i32> = make_i32(n);
        let dyn_items: Vec<&dyn ex15_spec::Summable> = values.iter().map(|v| v as &dyn ex15_spec::Summable).collect();
        let dyn_items_naive: Vec<&dyn ex15_naive::Summable> = values.iter().map(|v| v as &dyn ex15_naive::Summable).collect();
        let dyn_items_enum: Vec<&dyn ex15_enum::Summable> = values.iter().map(|v| v as &dyn ex15_enum::Summable).collect();
        let dyn_items_rt: Vec<&dyn ex15_runtime_typeid::Summable> = values.iter().map(|v| v as &dyn ex15_runtime_typeid::Summable).collect();
        g.throughput(Throughput::Elements(n as u64));
        let comp = ex15_spec::Computer;
        let comp_naive = ex15_naive::Computer;
        let comp_enum = ex15_enum::Computer;
        let comp_rt = ex15_runtime_typeid::Computer;
        g.bench_with_input(BenchmarkId::new("spec", n), &dyn_items, |b, items| {
            b.iter(|| ex15_spec::compute_sum_dyn(&comp, std::hint::black_box(items)))
        });
        g.bench_with_input(BenchmarkId::new("naive", n), &dyn_items_naive, |b, items| {
            b.iter(|| ex15_naive::compute_sum_naive(&comp_naive, std::hint::black_box(items)))
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &dyn_items_enum, |b, items| {
            b.iter(|| ex15_enum::compute_sum_enum(&comp_enum, std::hint::black_box(items)))
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &dyn_items_rt, |b, items| {
            b.iter(|| ex15_runtime_typeid::compute_sum_runtime(&comp_rt, std::hint::black_box(items)))
        });
    }
    g.finish();
}

// ── Ex16: total_capacity — wildcard + type alias ───────────────────────
fn bench_ex16(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex16_total_capacity");
    for &n in &SIZES {
        let items = make_vec_u8_items(n / 16, 16);
        g.throughput(Throughput::Elements(n as u64));
        let op = ex16_spec::CapacityOp;
        let op_naive = ex16_naive::CapacityOp;
        let op_enum = ex16_enum::CapacityOp;
        let op_rt = ex16_runtime_typeid::CapacityOp;
        g.bench_with_input(BenchmarkId::new("spec", n), &items, |b, items| {
            b.iter(|| ex16_spec::total_capacity_vec_u8(&op, std::hint::black_box(items)))
        });
        g.bench_with_input(BenchmarkId::new("naive", n), &items, |b, items| {
            b.iter(|| ex16_naive::total_capacity_naive(&op_naive, std::hint::black_box(items)))
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &items, |b, items| {
            b.iter(|| ex16_enum::total_capacity_enum(&op_enum, std::hint::black_box(items)))
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &items, |b, items| {
            b.iter(|| ex16_runtime_typeid::total_capacity_runtime(&op_rt, std::hint::black_box(items)))
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
    bench_ex09,
    bench_ex10,
    bench_ex11,
    bench_ex12,
    bench_ex13,
    bench_ex14,
    bench_ex15,
    bench_ex16,
);
criterion_main!(benches);
