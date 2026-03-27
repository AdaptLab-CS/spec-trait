use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use spec_trait_eval_benches::data::*;
use spec_trait_eval_benches::examples::*;
use spec_trait_eval_benches::examples::ex11_dyn::DynContains;
use spec_trait_eval_benches::examples::ex18_dyn::DynApply;
use spec_trait_eval_benches::examples::ex30_dyn::DynFilterCount;
use spec_trait_eval_benches::examples::ex31_dyn::DynCompute;
use spec_trait_eval_benches::examples::ex33_dyn::DynCount;
use spec_trait_eval_benches::examples::ex36_dyn::DynSortedCount;

const SIZES: [usize; 5] = [256, 1024, 4096, 16384, 65536];
fn bench_ex01(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex01_sum_i32");
    for &n in &SIZES {
        let data = make_i32(n);
        g.throughput(Throughput::Elements(n as u64));
        let s = ex01_spec::Summator;
        g.bench_with_input(BenchmarkId::new("spec", n), &data, |b, data| {
            b.iter(|| ex01_spec::sum_i32(&s, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("dyn", n), &data, |b, data| {
            b.iter(|| ex01_dyn::sum_dyn(std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &data, |b, data| {
            let e = ex01_enum::SumEnum::I32(data);
            b.iter(|| ex01_enum::sum_enum(&e))
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &data, |b, data| {
            b.iter(|| ex01_runtime_typeid::sum_runtime_i32(std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("generic", n), &data, |b, data| {
            b.iter(|| ex01_generic::sum_generic_i32(std::hint::black_box(data)))
        });
    }
    g.finish();
}

// ── Ex01: Abs ────────────────────────────────────────────────────────────
fn bench_ex02(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex02_abs_i32");
    for &n in &SIZES {
        let data = make_i32(n);
        g.throughput(Throughput::Elements(n as u64));
        let ac = ex02_spec::AbsComputer;
        g.bench_with_input(BenchmarkId::new("spec", n), &data, |b, data| {
            b.iter(|| ex02_spec::abs_i32(&ac, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("generic", n), &data, |b, data| {
            b.iter(|| ex02_spec::abs_generic(&ac, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &data, |b, data| {
            let e = ex02_enum::AbsEnum::I32(data);
            b.iter(|| ex02_enum::abs_enum_all_i32(&e))
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &data, |b, data| {
            b.iter(|| ex02_runtime_typeid::abs_runtime_i32(std::hint::black_box(data)))
        });
    }
    g.finish();
}


// ── Ex02: Hash ───────────────────────────────────────────────────────────
fn bench_ex03(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex03_hash_u8");
    for &n in &SIZES {
        let data = make_u8(n);
        g.throughput(Throughput::Elements(n as u64));
        let hc = ex03_spec::HashComputer;
        g.bench_with_input(BenchmarkId::new("spec", n), &data, |b, data| {
            b.iter(|| ex03_spec::hash_u8(&hc, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("generic", n), &data, |b, data| {
            b.iter(|| ex03_spec::hash_generic(&hc, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("dyn", n), &data, |b, data| {
            b.iter(|| ex03_dyn::hash_dyn(std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &data, |b, data| {
            let e = ex03_enum::HashEnum::U8(data);
            b.iter(|| ex03_enum::hash_enum_all(&e))
        });
    }
    g.finish();
}

// ── Ex03: Bulk serialize ─────────────────────────────────────────────────
fn bench_ex04(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex04_bulkser_u8");
    for &n in &SIZES {
        let data = make_u8(n);
        g.throughput(Throughput::Elements(n as u64));
        let bs = ex04_spec::BulkSerializer;
        g.bench_with_input(BenchmarkId::new("spec", n), &data, |b, data| {
            b.iter(|| ex04_spec::ser_bulk_u8(&bs, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("generic", n), &data, |b, data| {
            b.iter(|| ex04_spec::ser_bulk_generic(&bs, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &data, |b, data| {
            let e = ex04_enum::BulkSerEnum::U8(data);
            b.iter(|| ex04_enum::bulkser_enum_ser_bulk(&e))
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &data, |b, data| {
            b.iter(|| ex04_runtime_typeid::ser_bulk_runtime_u8(std::hint::black_box(data)))
        });
    }
    g.finish();
}

// ── Ex04: Copy vs Clone ──────────────────────────────────────────────────
fn bench_ex05(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex05_copy_i32");
    for &n in &SIZES {
        let data = make_i32(n);
        g.throughput(Throughput::Elements(n as u64));
        let cp = ex05_spec::Copier;
        g.bench_with_input(BenchmarkId::new("spec", n), &data, |b, data| {
            b.iter(|| ex05_spec::copy_i32(&cp, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("generic", n), &data, |b, data| {
            b.iter(|| ex05_spec::copy_generic(&cp, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("clone_fallback", n), &data, |b, data| {
            b.iter(|| ex05_generic::copy_generic(std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("manual_memcpy", n), &data, |b, data| {
            b.iter(|| ex05_generic::copy_memcpy_i32(std::hint::black_box(data)))
        });
    }
    g.finish();
}

// ── Ex05: Dedup ──────────────────────────────────────────────────────────
fn bench_ex06(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex06_dedup_u8");
    for &n in &SIZES {
        let data = make_sorted_u8(n);
        g.throughput(Throughput::Elements(n as u64));
        let dd = ex06_spec::Deduplicator;
        g.bench_with_input(BenchmarkId::new("spec", n), &data, |b, data| {
            b.iter(|| ex06_spec::dedup_u8(&dd, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("generic", n), &data, |b, data| {
            b.iter(|| ex06_spec::dedup_generic(&dd, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &data, |b, data| {
            let e = ex06_enum::DedupEnum::U8(data);
            b.iter(|| ex06_enum::dedup_enum_u8(&e))
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &data, |b, data| {
            b.iter(|| ex06_runtime_typeid::dedup_runtime_u8(std::hint::black_box(data)))
        });
    }
    g.finish();
}

// ── Ex06: Map double ─────────────────────────────────────────────────────
fn bench_ex07(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex07_map_i32");
    for &n in &SIZES {
        let data = make_i32(n);
        g.throughput(Throughput::Elements(n as u64));
        let m = ex07_spec::Mapper;
        g.bench_with_input(BenchmarkId::new("spec", n), &data, |b, data| {
            b.iter(|| ex07_spec::map_i32(&m, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("generic", n), &data, |b, data| {
            b.iter(|| ex07_spec::map_generic(&m, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &data, |b, data| {
            let e = ex07_enum::MapEnum::I32(data);
            b.iter(|| ex07_enum::map_enum_i32(&e))
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &data, |b, data| {
            b.iter(|| ex07_runtime_typeid::map_runtime_i32(std::hint::black_box(data)))
        });
    }
    g.finish();
}

// ── Ex07: Sort ───────────────────────────────────────────────────────────
fn bench_ex08(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex08_sort_u8");
    for &n in &SIZES {
        let base = make_u8(n);
        g.throughput(Throughput::Elements(n as u64));
        let sr = ex08_spec::Sorter;
        g.bench_with_input(BenchmarkId::new("spec", n), &base, |b, base| {
            b.iter(|| { let mut d = base.clone(); ex08_spec::sort_u8(&sr, std::hint::black_box(&mut d)); d })
        });
        g.bench_with_input(BenchmarkId::new("generic", n), &base, |b, base| {
            b.iter(|| { let mut d = base.clone(); ex08_spec::sort_generic(&sr, std::hint::black_box(&mut d)); d })
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &base, |b, base| {
            b.iter(|| { let mut d = base.clone(); ex08_runtime_typeid::sort_runtime_u8(std::hint::black_box(&mut d)); d })
        });
        g.bench_with_input(BenchmarkId::new("generic_stdlib", n), &base, |b, base| {
            b.iter(|| { let mut d = base.clone(); ex08_generic::sort_generic_u8(std::hint::black_box(&mut d)); d })
        });
        g.bench_with_input(BenchmarkId::new("manual_counting", n), &base, |b, base| {
            b.iter(|| { let mut d = base.clone(); ex08_compat::sort_counting_u8(std::hint::black_box(&mut d)); d })
        });
    }
    g.finish();
}

// ── Ex08: Format ─────────────────────────────────────────────────────────
fn bench_ex09(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex09_fmt_i32");
    let counts = [256, 1024, 4096];
    for &n in &counts {
        let data = make_i32(n);
        g.throughput(Throughput::Elements(n as u64));
        let f = ex09_spec::Formatter;
        g.bench_with_input(BenchmarkId::new("spec", n), &data, |b, data| {
            b.iter(|| ex09_spec::fmt_i32(&f, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("generic", n), &data, |b, data| {
            b.iter(|| ex09_spec::fmt_generic(&f, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &data, |b, data| {
            let e = ex09_enum::FmtEnum::I32(data);
            b.iter(|| ex09_enum::fmt_enum_all(&e))
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &data, |b, data| {
            b.iter(|| ex09_runtime_typeid::fmt_runtime(std::hint::black_box(data)))
        });
    }
    g.finish();
}

// ── Ex09: MinMax ─────────────────────────────────────────────────────────
fn bench_ex10(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex10_minmax_i32");
    for &n in &SIZES {
        let data = make_i32(n);
        g.throughput(Throughput::Elements(n as u64));
        let mm = ex10_spec::MinMaxer;
        g.bench_with_input(BenchmarkId::new("spec", n), &data, |b, data| {
            b.iter(|| ex10_spec::minmax_i32(&mm, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &data, |b, data| {
            let e = ex10_enum::MinMaxEnum::I32(data);
            b.iter(|| ex10_enum::minmax_enum_i32(&e))
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &data, |b, data| {
            b.iter(|| ex10_runtime_typeid::minmax_runtime_i32(std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("generic", n), &data, |b, data| {
            b.iter(|| ex10_generic::minmax_generic_i32(std::hint::black_box(data)))
        });
    }
    g.finish();
}

// ── Ex10: Contains ───────────────────────────────────────────────────────
fn bench_ex11(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex11_contains_u8");
    for &n in &SIZES {
        let data = make_u8(n);
        let needle: u8 = 200;
        g.throughput(Throughput::Elements(n as u64));
        let sc = ex11_spec::Searcher;
        g.bench_with_input(BenchmarkId::new("spec", n), &data, |b, data| {
            b.iter(|| ex11_spec::contains_u8(&sc, std::hint::black_box(data), &needle))
        });
        g.bench_with_input(BenchmarkId::new("generic", n), &data, |b, data| {
            b.iter(|| ex11_spec::contains_generic(&sc, std::hint::black_box(data), &needle))
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &data, |b, data| {
            let e = ex11_enum::ContainsEnum::U8(data, needle);
            b.iter(|| ex11_enum::contains_enum(&e))
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &data, |b, data| {
            b.iter(|| ex11_runtime_typeid::contains_runtime_u8(std::hint::black_box(data), &needle))
        });
        g.bench_with_input(BenchmarkId::new("dyn", n), &data, |b, data| {
            let s = ex11_dyn::DynU8Search { haystack: data, needle };
            b.iter(|| s.contains_dyn())
        });
    }
    g.finish();
}

// ── Ex11: Fill ───────────────────────────────────────────────────────────
fn bench_ex12(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex12_fill_u8");
    for &n in &SIZES {
        g.throughput(Throughput::Elements(n as u64));
        let fl = ex12_spec::Filler;
        g.bench_with_input(BenchmarkId::new("spec", n), &n, |b, &n| {
            let mut buf = vec![0u8; n];
            b.iter(|| ex12_spec::fill_u8(&fl, std::hint::black_box(&mut buf), 0x42))
        });
        g.bench_with_input(BenchmarkId::new("generic", n), &n, |b, &n| {
            let mut buf = vec![0u8; n];
            b.iter(|| ex12_spec::fill_generic(&fl, std::hint::black_box(&mut buf), 0x42))
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &n, |b, &n| {
            let mut buf = vec![0u8; n];
            b.iter(|| ex12_runtime_typeid::fill_runtime_u8(std::hint::black_box(&mut buf), 0x42))
        });
        g.bench_with_input(BenchmarkId::new("manual_memset", n), &n, |b, &n| {
            let mut buf = vec![0u8; n];
            b.iter(|| ex12_compat::fill_memset_u8(std::hint::black_box(&mut buf), 0x42))
        });
    }
    g.finish();
}

// ── Ex12: Clamp ──────────────────────────────────────────────────────────
fn bench_ex13(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex13_clamp_i32");
    for &n in &SIZES {
        let data = make_i32(n);
        g.throughput(Throughput::Elements(n as u64));
        let cl = ex13_spec::Clamper;
        g.bench_with_input(BenchmarkId::new("spec", n), &data, |b, data| {
            b.iter(|| ex13_spec::clamp_i32(&cl, std::hint::black_box(data), 100, 900))
        });
        g.bench_with_input(BenchmarkId::new("generic", n), &data, |b, data| {
            b.iter(|| ex13_spec::clamp_generic(&cl, std::hint::black_box(data), 100, 900))
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &data, |b, data| {
            let e = ex13_enum::ClampEnum::I32(data, 100, 900);
            b.iter(|| ex13_enum::clamp_enum_i32(&e))
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &data, |b, data| {
            b.iter(|| ex13_runtime_typeid::clamp_runtime_i32(std::hint::black_box(data), 100, 900))
        });
    }
    g.finish();
}

// ── Ex13: FMA ────────────────────────────────────────────────────────────
fn bench_ex14(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex14_fma_f64");
    for &n in &SIZES {
        let data = make_f64(n);
        g.throughput(Throughput::Elements(n as u64));
        let fc = ex14_spec::FmaComputer;
        g.bench_with_input(BenchmarkId::new("spec", n), &data, |b, data| {
            b.iter(|| ex14_spec::fma_f64(&fc, std::hint::black_box(data), 2.5, 1.0))
        });
        g.bench_with_input(BenchmarkId::new("generic", n), &data, |b, data| {
            b.iter(|| ex14_spec::fma_generic(&fc, std::hint::black_box(data), 2.5, 1.0))
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &data, |b, data| {
            let e = ex14_enum::FmaEnum::F64(data, 2.5, 1.0);
            b.iter(|| ex14_enum::fma_enum_f64(&e))
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &data, |b, data| {
            b.iter(|| ex14_runtime_typeid::fma_runtime_f64(std::hint::black_box(data), 2.5, 1.0))
        });
    }
    g.finish();
}

// ── Ex14: From bytes ─────────────────────────────────────────────────────
fn bench_ex15(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex15_frombytes_u8");
    for &n in &SIZES {
        let data = make_u8(n);
        g.throughput(Throughput::Elements(n as u64));
        let cv = ex15_spec::Converter;
        g.bench_with_input(BenchmarkId::new("spec", n), &data, |b, data| {
            b.iter(|| ex15_spec::from_bytes_u8(&cv, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("generic", n), &data, |b, data| {
            b.iter(|| ex15_generic::from_bytes_generic_u8(std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &data, |b, data| {
            let e = ex15_enum::FromBytesEnum::U8(data);
            b.iter(|| ex15_enum::frombytes_enum_u8(&e))
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &data, |b, data| {
            b.iter(|| ex15_runtime_typeid::from_bytes_runtime_u8(std::hint::black_box(data)))
        });
    }
    g.finish();
}

// ── Ex15: Dot product ────────────────────────────────────────────────────
fn bench_ex16(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex16_dot_f64");
    for &n in &SIZES {
        let a = make_f64(n);
        let b_data = make_f64(n);
        g.throughput(Throughput::Elements(n as u64));
        let dp = ex16_spec::DotProduct;
        g.bench_with_input(BenchmarkId::new("spec", n), &n, |bench, _| {
            bench.iter(|| ex16_spec::dot_f64(&dp, std::hint::black_box(&a), std::hint::black_box(&b_data)))
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &n, |bench, _| {
            let e = ex16_enum::DotEnum::F64(&a, &b_data);
            bench.iter(|| ex16_enum::dot_enum_f64(&e))
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &n, |bench, _| {
            bench.iter(|| ex16_runtime_typeid::dot_runtime_f64(std::hint::black_box(&a), std::hint::black_box(&b_data)))
        });
        g.bench_with_input(BenchmarkId::new("generic", n), &n, |bench, _| {
            bench.iter(|| ex16_generic::dot_generic_f64(std::hint::black_box(&a), std::hint::black_box(&b_data)))
        });
    }
    g.finish();
}

// ── Ex16: Lifetime specialization ────────────────────────────────────────
fn bench_ex17(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex17_lifetime_static");
    // Build a Vec of &'static str
    let static_strs: Vec<&'static str> = vec!["hello"; 65536];
    for &n in &SIZES {
        let items = &static_strs[..n];
        g.throughput(Throughput::Elements(n as u64));
        let p = ex17_spec::Parser;
        g.bench_with_input(BenchmarkId::new("spec_static", n), &items, |b, items| {
            b.iter(|| ex17_spec::sum_parse_static(&p, std::hint::black_box(items)))
        });
        g.bench_with_input(BenchmarkId::new("spec_borrowed", n), &items, |b, items| {
            b.iter(|| ex17_spec::sum_parse_borrowed(&p, std::hint::black_box(items)))
        });
        g.bench_with_input(BenchmarkId::new("spec_generic", n), &items, |b, items| {
            b.iter(|| ex17_spec::sum_parse_generic(&p, std::hint::black_box(items)))
        });
        g.bench_with_input(BenchmarkId::new("enum_static", n), &items, |b, items| {
            b.iter(|| ex17_compat::sum_parse_enum_static(std::hint::black_box(items)))
        });
        g.bench_with_input(BenchmarkId::new("dyn_static", n), &items, |b, items| {
            b.iter(|| ex17_compat::sum_parse_dyn_static(std::hint::black_box(items)))
        });
        g.bench_with_input(BenchmarkId::new("generic", n), &items, |b, items| {
            b.iter(|| ex17_compat::sum_parse_generic(std::hint::black_box(items)))
        });
    }
    g.finish();
}

// ── Ex17: HRTB — higher-order function application ───────────────────────
fn bench_ex18(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex18_hrtb_apply");
    fn double_i32(x: &i32) -> i32 { x * 2 }
    for &n in &SIZES {
        let data = make_i32(n);
        g.throughput(Throughput::Elements(n as u64));
        let a = ex18_spec::Applicator;
        g.bench_with_input(BenchmarkId::new("spec_hrtb", n), &data, |b, data| {
            b.iter(|| ex18_spec::apply_i32_fn(&a, double_i32, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("spec_generic", n), &data, |b, data| {
            b.iter(|| ex18_spec::apply_generic(&a, double_i32, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &data, |b, data| {
            let e = ex18_enum::ApplyEnum::I32(double_i32, data);
            b.iter(|| ex18_enum::apply_enum_i32(&e))
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &data, |b, data| {
            b.iter(|| ex18_runtime_typeid::apply_runtime_i32(double_i32, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("generic", n), &data, |b, data| {
            b.iter(|| ex18_generic::apply_generic(double_i32, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("dyn", n), &data, |b, data| {
            let d = ex18_dyn::DynI32Apply { f: double_i32, data };
            b.iter(|| d.apply_dyn())
        });
    }
    g.finish();
}

// ── Ex18: Multi-generic repeating ────────────────────────────────────────
fn bench_ex19(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex19_multi_generic");
    for &n in &SIZES {
        let xs = make_u8(n);
        let ys: Vec<Vec<u8>> = (0..n).map(|i| vec![(i & 0xFF) as u8; 4]).collect();
        g.throughput(Throughput::Elements(n as u64));
        let comb = ex19_spec::Combiner;
        g.bench_with_input(BenchmarkId::new("spec_u8_vecu8", n), &n, |b, _| {
            b.iter(|| ex19_spec::combine_u8_vecu8(&comb, std::hint::black_box(&xs), std::hint::black_box(&ys)))
        });
        g.bench_with_input(BenchmarkId::new("spec_generic", n), &n, |b, _| {
            b.iter(|| ex19_spec::combine_generic(&comb, std::hint::black_box(&xs), std::hint::black_box(&ys)))
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &n, |b, _| {
            let e = ex19_enum::CombineEnum::U8VecU8(&xs, &ys);
            b.iter(|| ex19_enum::combine_enum(&e))
        });
        g.bench_with_input(BenchmarkId::new("manual", n), &n, |b, _| {
            b.iter(|| ex19_compat::combine_u8_vecu8(std::hint::black_box(&xs), std::hint::black_box(&ys)))
        });
        g.bench_with_input(BenchmarkId::new("generic", n), &n, |b, _| {
            b.iter(|| ex19_generic::combine_generic(std::hint::black_box(&xs), std::hint::black_box(&ys)))
        });
    }
    g.finish();
}

// ── Ex19: Wildcard & complex type patterns ───────────────────────────────
fn bench_ex20(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex20_wildcard_types");
    for &n in &SIZES {
        let items: Vec<Vec<i32>> = (0..n).map(|i| vec![i as i32; 4]).collect();
        g.throughput(Throughput::Elements(n as u64));
        let m = ex20_spec::Measurer;
        g.bench_with_input(BenchmarkId::new("spec_vec_wildcard", n), &n, |b, _| {
            b.iter(|| ex20_spec::sum_measure_vec_i32(&m, std::hint::black_box(&items)))
        });
        g.bench_with_input(BenchmarkId::new("spec_generic", n), &n, |b, _| {
            b.iter(|| ex20_spec::sum_measure_generic(&m, std::hint::black_box(&items)))
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &n, |b, _| {
            b.iter(|| ex20_compat::sum_measure_enum_vec_i32(std::hint::black_box(&items)))
        });
        g.bench_with_input(BenchmarkId::new("manual", n), &n, |b, _| {
            b.iter(|| ex20_compat::sum_measure_manual(std::hint::black_box(&items)))
        });
        g.bench_with_input(BenchmarkId::new("generic", n), &n, |b, _| {
            b.iter(|| ex20_compat::sum_measure_generic(std::hint::black_box(&items)))
        });
    }
    g.finish();
}

// ── Ex20: Conditional combinators (any/all/not) ──────────────────────────
fn bench_ex21(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex21_conditional");
    for &n in &SIZES {
        let data_i32 = make_i32(n);
        let data_u8 = make_u8(n);
        g.throughput(Throughput::Elements(n as u64));
        let p = ex21_spec::Processor;
        g.bench_with_input(BenchmarkId::new("spec_any_i32", n), &data_i32, |b, data| {
            b.iter(|| ex21_spec::process_i32(&p, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("spec_not_u8", n), &data_u8, |b, data| {
            b.iter(|| ex21_spec::process_u8(&p, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("spec_generic", n), &data_i32, |b, data| {
            b.iter(|| ex21_spec::process_generic(&p, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("enum_i32", n), &data_i32, |b, data| {
            let e = ex21_enum::ProcessEnum::I32(data);
            b.iter(|| ex21_enum::process_enum(&e))
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &data_i32, |b, data| {
            b.iter(|| ex21_runtime_typeid::process_runtime_i32(std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("manual_i32", n), &data_i32, |b, data| {
            b.iter(|| ex21_compat::process_manual_i32(std::hint::black_box(data)))
        });
    }
    g.finish();
}

// ── Ex21: Fn trait bound specialization ──────────────────────────────────
fn bench_ex22(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex22_fn_trait_bounds");
    fn add_one(x: i32) -> i32 { x + 1 }
    fn deref_add(x: &i32) -> i32 { *x + 1 }
    let fns_i32: Vec<fn(i32) -> i32> = vec![add_one; 65536];
    let fns_ref: Vec<fn(&i32) -> i32> = vec![deref_add; 65536];
    for &n in &SIZES {
        g.throughput(Throughput::Elements(n as u64));
        let d = ex22_spec::Dispatcher;
        let slice_i32 = &fns_i32[..n];
        let slice_ref = &fns_ref[..n];
        g.bench_with_input(BenchmarkId::new("spec_fn_i32", n), &n, |b, _| {
            b.iter(|| ex22_spec::sum_dispatch_fn_i32(&d, std::hint::black_box(slice_i32)))
        });
        g.bench_with_input(BenchmarkId::new("spec_fn_ref_i32", n), &n, |b, _| {
            b.iter(|| ex22_spec::sum_dispatch_fn_ref_i32(&d, std::hint::black_box(slice_ref)))
        });
        g.bench_with_input(BenchmarkId::new("spec_generic", n), &n, |b, _| {
            b.iter(|| ex22_spec::sum_dispatch_generic(&d, std::hint::black_box(slice_i32)))
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &n, |b, _| {
            b.iter(|| ex22_compat::sum_dispatch_enum(std::hint::black_box(slice_i32)))
        });
        g.bench_with_input(BenchmarkId::new("manual", n), &n, |b, _| {
            b.iter(|| ex22_compat::sum_dispatch_manual(std::hint::black_box(slice_i32)))
        });
        g.bench_with_input(BenchmarkId::new("dyn", n), &n, |b, _| {
            b.iter(|| ex22_compat::sum_dispatch_dyn(std::hint::black_box(slice_i32)))
        });
    }
    g.finish();
}

// ── Ex22: Multiple self types ────────────────────────────────────────────
fn bench_ex23(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex23_multi_self");
    for &n in &SIZES {
        let data = make_u8(n);
        g.throughput(Throughput::Elements(n as u64));
        let a = ex23_spec::TypeA;
        let b_type = ex23_spec::TypeB;
        g.bench_with_input(BenchmarkId::new("spec_typeA_u8", n), &data, |b, data| {
            b.iter(|| ex23_spec::sum_transform_a_u8(&a, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("spec_typeB_u8", n), &data, |b, data| {
            b.iter(|| ex23_spec::sum_transform_b_u8(&b_type, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("spec_generic", n), &data, |b, data| {
            b.iter(|| ex23_spec::sum_transform_generic_a(&a, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &data, |b, data| {
            b.iter(|| ex23_compat::sum_transform_enum_u8(std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("manual", n), &data, |b, data| {
            b.iter(|| ex23_compat::sum_transform_manual_u8(std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("dyn", n), &data, |b, data| {
            b.iter(|| ex23_compat::sum_transform_dyn(std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("generic", n), &data, |b, data| {
            b.iter(|| ex23_compat::sum_transform_generic(std::hint::black_box(data)))
        });
    }
    g.finish();
}

// ── Ex23: 'static lifetime bound ─────────────────────────────────────────
fn bench_ex24(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex24_static_bound");
    for &n in &SIZES {
        let data = make_i32(n);
        g.throughput(Throughput::Elements(n as u64));
        let h = ex24_spec::Hasher;
        g.bench_with_input(BenchmarkId::new("spec", n), &data, |b, data| {
            b.iter(|| ex24_spec::sum_hash_i32(&h, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("generic", n), &data, |b, data| {
            b.iter(|| ex24_spec::sum_hash_generic(&h, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &data, |b, data| {
            b.iter(|| ex24_compat::sum_hash_enum(std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &data, |b, data| {
            b.iter(|| ex24_compat::sum_hash_runtime(std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("dyn", n), &data, |b, data| {
            b.iter(|| ex24_compat::sum_hash_dyn(std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("manual", n), &data, |b, data| {
            b.iter(|| ex24_compat::sum_hash_manual(std::hint::black_box(data)))
        });
    }
    g.finish();
}

// ── Ex24: &dyn Trait type pattern ────────────────────────────────────────
fn bench_ex25(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex25_dyn_trait_type");
    for &n in &SIZES {
        let data = make_i32(n);
        g.throughput(Throughput::Elements(n as u64));
        let r = ex25_spec::Renderer;
        g.bench_with_input(BenchmarkId::new("spec", n), &data, |b, data| {
            b.iter(|| ex25_spec::sum_render_i32(&r, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("generic", n), &data, |b, data| {
            b.iter(|| ex25_spec::sum_render_generic(&r, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &data, |b, data| {
            b.iter(|| ex25_compat::sum_render_enum(std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &data, |b, data| {
            b.iter(|| ex25_compat::sum_render_runtime(std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("dyn", n), &data, |b, data| {
            b.iter(|| ex25_compat::sum_render_dyn(std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("manual", n), &data, |b, data| {
            b.iter(|| ex25_compat::sum_render_manual(std::hint::black_box(data)))
        });
    }
    g.finish();
}

// ── Ex25: Combined trait bounds (Copy + Eq) ──────────────────────────────
fn bench_ex26(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex26_combined_bounds");
    // Use small chunks so the O(n²) generic doesn't dominate too much
    let small_sizes: [usize; 4] = [16, 64, 128, 256];
    for &n in &small_sizes {
        let data = make_u8(n);
        let chunks: Vec<&[u8]> = data.chunks(n.min(64)).collect();
        g.throughput(Throughput::Elements(n as u64));
        let d = ex26_spec::Deduper;
        g.bench_with_input(BenchmarkId::new("spec_u8", n), &n, |b, _| {
            b.iter(|| ex26_spec::sum_dedup_u8(&d, std::hint::black_box(&chunks)))
        });
        g.bench_with_input(BenchmarkId::new("generic", n), &n, |b, _| {
            b.iter(|| ex26_spec::sum_dedup_generic(&d, std::hint::black_box(&chunks)))
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &n, |b, _| {
            b.iter(|| ex26_compat::sum_dedup_enum_u8(std::hint::black_box(&chunks)))
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &n, |b, _| {
            b.iter(|| ex26_compat::sum_dedup_runtime_u8(std::hint::black_box(&chunks)))
        });
        g.bench_with_input(BenchmarkId::new("manual", n), &n, |b, _| {
            b.iter(|| ex26_compat::sum_dedup_manual_u8(std::hint::black_box(&chunks)))
        });
    }
    g.finish();
}

// ── Ex26: Many specializations (10 types) ────────────────────────────────
fn bench_ex27(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex27_many_specs");
    for &n in &SIZES {
        let data_i32 = make_i32(n);
        let chunks_i32: Vec<&[i32]> = data_i32.chunks(n.min(1024)).collect();
        g.throughput(Throughput::Elements(n as u64));
        let e = ex27_spec::Encoder;
        g.bench_with_input(BenchmarkId::new("spec_i32", n), &n, |b, _| {
            b.iter(|| ex27_spec::sum_encode_i32(&e, std::hint::black_box(&chunks_i32)))
        });
        g.bench_with_input(BenchmarkId::new("generic", n), &n, |b, _| {
            b.iter(|| ex27_spec::sum_encode_generic(&e, std::hint::black_box(&chunks_i32)))
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &n, |b, _| {
            b.iter(|| ex27_compat::sum_encode_enum_i32(std::hint::black_box(&chunks_i32)))
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &n, |b, _| {
            b.iter(|| ex27_compat::sum_encode_runtime_i32(std::hint::black_box(&chunks_i32)))
        });
        g.bench_with_input(BenchmarkId::new("manual", n), &n, |b, _| {
            b.iter(|| ex27_compat::sum_encode_manual_i32(std::hint::black_box(&chunks_i32)))
        });
        g.bench_with_input(BenchmarkId::new("dyn", n), &n, |b, _| {
            b.iter(|| ex27_compat::sum_encode_dyn_i32(std::hint::black_box(&chunks_i32)))
        });
    }
    g.finish();
}

// ── Ex27: Tuple type specialization ──────────────────────────────────────
fn bench_ex28(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex28_tuple_types");
    for &n in &SIZES {
        let data_i32: Vec<(i32, i32)> = (0..n).map(|i| (i as i32, (i * 2) as i32)).collect();
        let data_f64: Vec<(f64, f64)> = (0..n).map(|i| (i as f64, (i * 2) as f64)).collect();
        g.throughput(Throughput::Elements(n as u64));
        let ops = ex28_spec::PointOps;
        g.bench_with_input(BenchmarkId::new("spec_i32", n), &data_i32, |b, data| {
            b.iter(|| ex28_spec::squared_sum_i32(&ops, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("spec_f64", n), &data_f64, |b, data| {
            b.iter(|| ex28_spec::squared_sum_f64(&ops, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("generic", n), &data_i32, |b, data| {
            b.iter(|| ex28_generic::squared_sum_generic_i32(std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("enum_i32", n), &data_i32, |b, data| {
            let e = ex28_enum::PointEnum::I32(data);
            b.iter(|| ex28_enum::squared_sum_enum(&e))
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &data_i32, |b, data| {
            b.iter(|| ex28_runtime_typeid::squared_sum_runtime_i32(std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("manual", n), &data_i32, |b, data| {
            b.iter(|| ex28_generic::squared_sum_manual_i32(std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("dyn", n), &data_i32, |b, data| {
            let d = ex28_dyn::DynI32Points(data);
            let dd: &dyn ex28_dyn::DynSquaredSum = &d;
            b.iter(|| dd.squared_sum_dyn())
        });
    }
    g.finish();
}

// ── Ex28: Nested generic types ───────────────────────────────────────────
fn bench_ex29(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex29_nested_generics");
    for &n in &SIZES {
        let data_vvi: Vec<Vec<Vec<u8>>> = (0..n.min(1024))
            .map(|i| vec![vec![(i & 0xFF) as u8; 4]; 2])
            .collect();
        let data_vi: Vec<Vec<i32>> = (0..n.min(1024))
            .map(|i| vec![i as i32; 4])
            .collect();
        let elem_count = n.min(1024);
        g.throughput(Throughput::Elements(elem_count as u64));
        let f = ex29_spec::Flattener;
        // Keep IDs unique across the whole group, even when data size is capped.
        g.bench_with_input(BenchmarkId::new("spec_vec_vec_u8", n), &elem_count, |b, _| {
            b.iter(|| ex29_spec::flat_len_vec_vec_u8(&f, std::hint::black_box(&data_vvi)))
        });
        g.bench_with_input(BenchmarkId::new("spec_vec_i32", n), &elem_count, |b, _| {
            b.iter(|| ex29_spec::flat_len_vec_i32(&f, std::hint::black_box(&data_vi)))
        });
        g.bench_with_input(BenchmarkId::new("generic", n), &elem_count, |b, _| {
            b.iter(|| ex29_generic::flat_generic_vec_vec_u8(std::hint::black_box(&data_vvi)))
        });
        g.bench_with_input(BenchmarkId::new("enum_vec_vec_u8", n), &elem_count, |b, _| {
            let e = ex29_enum::FlatEnum::VecVecU8(&data_vvi);
            b.iter(|| ex29_enum::flat_enum(&e))
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &elem_count, |b, _| {
            b.iter(|| ex29_runtime_typeid::flat_runtime_vec_vec_u8(std::hint::black_box(&data_vvi)))
        });
        g.bench_with_input(BenchmarkId::new("manual", n), &elem_count, |b, _| {
            b.iter(|| ex29_generic::flat_manual_vec_vec_u8(std::hint::black_box(&data_vvi)))
        });
        g.bench_with_input(BenchmarkId::new("dyn", n), &elem_count, |b, _| {
            let d = ex29_dyn::DynVecVecU8(&data_vvi);
            let dd: &dyn ex29_dyn::DynFlat = &d;
            b.iter(|| dd.flat_dyn())
        });
    }
    g.finish();
}

// ── Ex29: any() predicate – integer fast path ────────────────────────────
fn bench_ex30(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex30_any_pred");
    for &n in &SIZES {
        let data = make_i32(n);
        g.throughput(Throughput::Elements(n as u64));
        let f = ex30_spec::Filter;
        g.bench_with_input(BenchmarkId::new("spec", n), &data, |b, data| {
            b.iter(|| ex30_spec::filter_count_i32(&f, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("generic", n), &data, |b, data| {
            b.iter(|| ex30_spec::filter_count_generic(&f, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("dyn", n), &data, |b, data| {
            let d = ex30_dyn::DynFilterI32(data);
            b.iter(|| d.filter_count_dyn())
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &data, |b, data| {
            let e = ex30_enum::FilterEnum::I32(data);
            b.iter(|| ex30_enum::filter_count_enum(&e))
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &data, |b, data| {
            b.iter(|| ex30_runtime_typeid::filter_count_runtime_i32(std::hint::black_box(data)))
        });
    }
    g.finish();
}

// ── Ex30: not() predicate – type exclusion ───────────────────────────────
fn bench_ex31(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex31_not_pred");
    for &n in &SIZES {
        let data = make_i32(n);
        g.throughput(Throughput::Elements(n as u64));
        let nm = ex31_spec::Normalizer;
        g.bench_with_input(BenchmarkId::new("spec", n), &data, |b, data| {
            b.iter(|| ex31_spec::compute_i32(&nm, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("generic", n), &data, |b, data| {
            b.iter(|| ex31_spec::compute_generic(&nm, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("dyn", n), &data, |b, data| {
            let d = ex31_dyn::DynComputeI32(data);
            b.iter(|| d.compute_dyn())
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &data, |b, data| {
            let e = ex31_enum::NormEnum::I32(data);
            b.iter(|| ex31_enum::compute_enum(&e))
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &data, |b, data| {
            b.iter(|| ex31_runtime_typeid::compute_runtime_i32(std::hint::black_box(data)))
        });
    }
    g.finish();
}

// ── Ex31: all() compound predicate with nested any() ─────────────────────
fn bench_ex32(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex32_all_compound");
    for &n in &SIZES {
        let data = make_i32(n);
        g.throughput(Throughput::Elements(n as u64));
        let f = ex32_spec::Folder;
        g.bench_with_input(BenchmarkId::new("spec", n), &data, |b, data| {
            b.iter(|| ex32_spec::fold_count_i32(&f, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("generic", n), &data, |b, data| {
            b.iter(|| ex32_spec::fold_count_generic(&f, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &data, |b, data| {
            let e = ex32_enum::FoldEnum::I32(data);
            b.iter(|| ex32_enum::fold_count_enum(&e))
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &data, |b, data| {
            b.iter(|| ex32_runtime_typeid::fold_count_runtime_i32(std::hint::black_box(data)))
        });
    }
    g.finish();
}

// ── Ex32: Multiple receiver types ────────────────────────────────────────
fn bench_ex33(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex33_multi_recv");
    for &n in &SIZES {
        let data = make_u8(n);
        g.throughput(Throughput::Elements(n as u64));
        let vc = ex33_spec::VecCounter;
        g.bench_with_input(BenchmarkId::new("spec_vec", n), &data, |b, data| {
            b.iter(|| ex33_spec::count_vec_u8(&vc, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("generic", n), &data, |b, data| {
            b.iter(|| ex33_spec::count_vec_generic(&vc, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("dyn", n), &data, |b, data| {
            let d = ex33_dyn::DynVecCounterU8(data);
            b.iter(|| d.count_dyn())
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &data, |b, data| {
            let e = ex33_enum::CountEnum::VecU8(data);
            b.iter(|| ex33_enum::count_enum(&e))
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &data, |b, data| {
            b.iter(|| ex33_runtime_typeid::count_runtime_vec_u8(std::hint::black_box(data)))
        });
    }
    g.finish();
}

// ── Ex33: 'static + Copy compound bound ──────────────────────────────────
fn bench_ex34(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex34_static_copy");
    for &n in &SIZES {
        let data = make_i32(n);
        g.throughput(Throughput::Elements(n as u64));
        let ch = ex34_spec::Cacher;
        g.bench_with_input(BenchmarkId::new("spec", n), &data, |b, data| {
            b.iter(|| ex34_spec::cached_size_i32(&ch, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("generic", n), &data, |b, data| {
            b.iter(|| ex34_spec::cached_size_generic(&ch, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &data, |b, data| {
            let e = ex34_enum::CacheEnum::I32(data);
            b.iter(|| ex34_enum::cached_size_enum(&e))
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &data, |b, data| {
            b.iter(|| ex34_runtime_typeid::cached_size_runtime_i32(std::hint::black_box(data)))
        });
    }
    g.finish();
}

// ── Ex34: Two-generic concrete pair specialization ───────────────────────
fn bench_ex35(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex35_pair_spec");
    for &n in &SIZES {
        let ts = make_i32(n);
        let us = make_u8(n);
        g.throughput(Throughput::Elements(n as u64));
        let m = ex35_spec::Merger;
        g.bench_with_input(BenchmarkId::new("spec", n), &n, |b, _| {
            b.iter(|| ex35_spec::merge_i32_u8(&m, std::hint::black_box(&ts), std::hint::black_box(&us)))
        });
        g.bench_with_input(BenchmarkId::new("generic", n), &n, |b, _| {
            b.iter(|| ex35_spec::merge_generic(&m, std::hint::black_box(&ts), std::hint::black_box(&us)))
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &n, |b, _| {
            let e = ex35_enum::MergeEnum::I32U8(&ts, &us);
            b.iter(|| ex35_enum::merge_count_enum(&e))
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &n, |b, _| {
            b.iter(|| ex35_runtime_typeid::merge_count_runtime_i32_u8(std::hint::black_box(&ts), std::hint::black_box(&us)))
        });
    }
    g.finish();
}

// ── Ex35: Triple trait bounds (Clone + Eq + Ord) ─────────────────────────
fn bench_ex36(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex36_triple_bounds");
    for &n in &SIZES {
        let n_capped = n.min(4096);
        let data: Vec<i32> = (0..n_capped).map(|i| (i % 256) as i32).collect();
        g.throughput(Throughput::Elements(n_capped as u64));
        let s = ex36_spec::UniqueSorter;
        g.bench_with_input(BenchmarkId::new("spec", n), &data, |b, data| {
            b.iter(|| ex36_spec::sorted_unique_count_i32(&s, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("generic", n), &data, |b, data| {
            b.iter(|| ex36_spec::sorted_unique_count_generic(&s, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("dyn", n), &data, |b, data| {
            let d = ex36_dyn::DynSortI32(data);
            b.iter(|| d.sorted_unique_count_dyn())
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &data, |b, data| {
            let e = ex36_enum::SortEnum::I32(data);
            b.iter(|| ex36_enum::sorted_unique_count_enum(&e))
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &data, |b, data| {
            b.iter(|| ex36_runtime_typeid::sorted_unique_count_runtime_i32(std::hint::black_box(data)))
        });
    }
    g.finish();
}

// ── Ex36: Generic receiver + type specialization ─────────────────────────
fn bench_ex37(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex37_generic_recv");
    for &n in &SIZES {
        let data = make_i32(n);
        g.throughput(Throughput::Elements(n as u64));
        g.bench_with_input(BenchmarkId::new("spec", n), &data, |b, data| {
            b.iter(|| ex37_spec::accum_i32(std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("generic", n), &data, |b, data| {
            b.iter(|| ex37_spec::accum_generic(std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &data, |b, data| {
            let e = ex37_enum::AccumEnum::I32(data);
            b.iter(|| ex37_enum::accum_count_enum(&e))
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &data, |b, data| {
            b.iter(|| ex37_runtime_typeid::accum_count_runtime_i32(std::hint::black_box(data)))
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
    bench_ex17,
    bench_ex18,
    bench_ex19,
    bench_ex20,
    bench_ex21,
    bench_ex22,
    bench_ex23,
    bench_ex24,
    bench_ex25,
    bench_ex26,
    bench_ex27,
    bench_ex28,
    bench_ex29,
    bench_ex30,
    bench_ex31,
    bench_ex32,
    bench_ex33,
    bench_ex34,
    bench_ex35,
    bench_ex36,
    bench_ex37,
);
criterion_main!(benches);
