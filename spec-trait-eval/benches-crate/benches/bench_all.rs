use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use spec_trait_eval_benches::data::*;
use spec_trait_eval_benches::examples::*;
use spec_trait_eval_benches::examples::ex15_wo::DynContains;
use spec_trait_eval_benches::examples::ex22_wo::DynApply;

const SIZES: [usize; 5] = [256, 1024, 4096, 16384, 65536];

// ── Ex01: Saturating double ──────────────────────────────────────────────
fn bench_ex01(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex01_double_u8");
    for &n in &SIZES {
        let data = make_u8(n);
        g.throughput(Throughput::Elements(n as u64));
        let d = ex01::Doubler;
        g.bench_with_input(BenchmarkId::new("spec", n), &data, |b, data| {
            b.iter(|| ex01::double_u8(&d, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("generic", n), &data, |b, data| {
            b.iter(|| ex01::double_generic(&d, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &data, |b, data| {
            let e = ex01_wo::DoubleEnum::U8;
            b.iter(|| ex01_wo::double_enum_u8(&e, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &data, |b, data| {
            b.iter(|| ex01_wo::double_runtime_u8(std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("dyn", n), &data, |b, data| {
            let p: &dyn ex01_wo::DynDouble = &ex01_wo::DynU8Doubler;
            b.iter(|| p.double_dyn(std::hint::black_box(data)))
        });
    }
    g.finish();
}

// ── Ex02: Bit count ──────────────────────────────────────────────────────
fn bench_ex02(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex02_bitcount_u8");
    for &n in &SIZES {
        let data = make_u8(n);
        g.throughput(Throughput::Elements(n as u64));
        let ct = ex02::Counter;
        g.bench_with_input(BenchmarkId::new("spec", n), &data, |b, data| {
            b.iter(|| ex02::count_u8(&ct, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("generic", n), &data, |b, data| {
            b.iter(|| ex02::count_generic(&ct, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("dyn", n), &data, |b, data| {
            b.iter(|| ex02_wo::count_dyn(std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &data, |b, data| {
            let e = ex02_wo::BitCountEnum::U8(data);
            b.iter(|| ex02_wo::bitcount_enum_count_ones(&e))
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &data, |b, data| {
            b.iter(|| ex02_wo::count_runtime_u8(std::hint::black_box(data)))
        });
    }
    g.finish();
}

// ── Ex03: Sum ────────────────────────────────────────────────────────────
fn bench_ex03(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex03_sum_i32");
    for &n in &SIZES {
        let data = make_i32(n);
        g.throughput(Throughput::Elements(n as u64));
        let s = ex03::Summator;
        g.bench_with_input(BenchmarkId::new("spec", n), &data, |b, data| {
            b.iter(|| ex03::sum_i32(&s, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("dyn", n), &data, |b, data| {
            b.iter(|| ex03_wo::sum_dyn(std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &data, |b, data| {
            let e = ex03_wo::SumEnum::I32(data);
            b.iter(|| ex03_wo::sum_enum(&e))
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &data, |b, data| {
            b.iter(|| ex03_wo::sum_runtime_i32(std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("generic", n), &data, |b, data| {
            b.iter(|| ex03_wo::sum_generic_i32(std::hint::black_box(data)))
        });
    }
    g.finish();
}

// ── Ex04: Abs ────────────────────────────────────────────────────────────
fn bench_ex04(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex04_abs_i32");
    for &n in &SIZES {
        let data = make_i32(n);
        g.throughput(Throughput::Elements(n as u64));
        let ac = ex04::AbsComputer;
        g.bench_with_input(BenchmarkId::new("spec", n), &data, |b, data| {
            b.iter(|| ex04::abs_i32(&ac, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("generic", n), &data, |b, data| {
            b.iter(|| ex04::abs_generic(&ac, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &data, |b, data| {
            let e = ex04_wo::AbsEnum::I32(data);
            b.iter(|| ex04_wo::abs_enum_all_i32(&e))
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &data, |b, data| {
            b.iter(|| ex04_wo::abs_runtime_i32(std::hint::black_box(data)))
        });
    }
    g.finish();
}

// ── Ex05: Serialize ──────────────────────────────────────────────────────
fn bench_ex05(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex05_serialize_i32");
    let counts = [1000, 10_000, 100_000];
    for &n in &counts {
        let vals: Vec<i32> = (0..n as i32).collect();
        g.throughput(Throughput::Elements(n as u64));
        let ser = ex05::Serializer;
        g.bench_with_input(BenchmarkId::new("spec", n), &vals, |b, vals| {
            b.iter(|| { let mut t = 0usize; for v in vals { t += ex05::ser_i32(&ser, std::hint::black_box(v)).len(); } t })
        });
        g.bench_with_input(BenchmarkId::new("generic", n), &vals, |b, vals| {
            b.iter(|| { let mut t = 0usize; for v in vals { t += ex05::ser_generic(&ser, std::hint::black_box(v)).len(); } t })
        });
        g.bench_with_input(BenchmarkId::new("dyn", n), &vals, |b, vals| {
            b.iter(|| { let mut t = 0usize; for v in vals { t += ex05_wo::ser_dyn(std::hint::black_box(v as &dyn ex05_wo::DynSerialize)).len(); } t })
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &vals, |b, vals| {
            let payloads: Vec<_> = vals.iter().map(|v| ex05_wo::SerEnum::I32(*v)).collect();
            b.iter(|| { let mut t = 0usize; for p in &payloads { t += ex05_wo::ser_enum_serialize(p).len(); } t })
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &vals, |b, vals| {
            b.iter(|| { let mut t = 0usize; for v in vals { t += ex05_wo::ser_runtime(std::hint::black_box(v)).len(); } t })
        });
    }
    g.finish();
}

// ── Ex06: ToString ───────────────────────────────────────────────────────
fn bench_ex06(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex06_tostr_i32");
    let counts = [1000, 10_000, 100_000];
    for &n in &counts {
        let vals: Vec<i32> = (0..n as i32).collect();
        g.throughput(Throughput::Elements(n as u64));
        let st = ex06::Stringer;
        g.bench_with_input(BenchmarkId::new("spec", n), &vals, |b, vals| {
            b.iter(|| { let mut t = 0usize; for v in vals { t += ex06::tostr_i32(&st, std::hint::black_box(v)).len(); } t })
        });
        g.bench_with_input(BenchmarkId::new("generic", n), &vals, |b, vals| {
            b.iter(|| { let mut t = 0usize; for v in vals { t += ex06::tostr_generic(&st, std::hint::black_box(v)).len(); } t })
        });
        g.bench_with_input(BenchmarkId::new("dyn", n), &vals, |b, vals| {
            b.iter(|| { let mut t = 0usize; for v in vals { t += ex06_wo::tostr_dyn(std::hint::black_box(v as &dyn ex06_wo::DynToStr)).len(); } t })
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &vals, |b, vals| {
            let payloads: Vec<_> = vals.iter().map(|v| ex06_wo::ToStrEnum::I32(*v)).collect();
            b.iter(|| { let mut t = 0usize; for p in &payloads { t += ex06_wo::tostr_enum(p).len(); } t })
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &vals, |b, vals| {
            b.iter(|| { let mut t = 0usize; for v in vals { t += ex06_wo::tostr_runtime(std::hint::black_box(v)).len(); } t })
        });
    }
    g.finish();
}

// ── Ex07: Hash ───────────────────────────────────────────────────────────
fn bench_ex07(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex07_hash_u8");
    for &n in &SIZES {
        let data = make_u8(n);
        g.throughput(Throughput::Elements(n as u64));
        let hc = ex07::HashComputer;
        g.bench_with_input(BenchmarkId::new("spec", n), &data, |b, data| {
            b.iter(|| ex07::hash_u8(&hc, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("generic", n), &data, |b, data| {
            b.iter(|| ex07::hash_generic(&hc, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("dyn", n), &data, |b, data| {
            b.iter(|| ex07_wo::hash_dyn(std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &data, |b, data| {
            let e = ex07_wo::HashEnum::U8(data);
            b.iter(|| ex07_wo::hash_enum_all(&e))
        });
    }
    g.finish();
}

// ── Ex08: Bulk serialize ─────────────────────────────────────────────────
fn bench_ex08(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex08_bulkser_u8");
    for &n in &SIZES {
        let data = make_u8(n);
        g.throughput(Throughput::Elements(n as u64));
        let bs = ex08::BulkSerializer;
        g.bench_with_input(BenchmarkId::new("spec", n), &data, |b, data| {
            b.iter(|| ex08::ser_bulk_u8(&bs, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("generic", n), &data, |b, data| {
            b.iter(|| ex08::ser_bulk_generic(&bs, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &data, |b, data| {
            let e = ex08_wo::BulkSerEnum::U8(data);
            b.iter(|| ex08_wo::bulkser_enum_ser_bulk(&e))
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &data, |b, data| {
            b.iter(|| ex08_wo::ser_bulk_runtime_u8(std::hint::black_box(data)))
        });
    }
    g.finish();
}

// ── Ex09: Copy vs Clone ──────────────────────────────────────────────────
fn bench_ex09(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex09_copy_i32");
    for &n in &SIZES {
        let data = make_i32(n);
        g.throughput(Throughput::Elements(n as u64));
        let cp = ex09::Copier;
        g.bench_with_input(BenchmarkId::new("spec", n), &data, |b, data| {
            b.iter(|| ex09::copy_i32(&cp, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("generic", n), &data, |b, data| {
            b.iter(|| ex09::copy_generic(&cp, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("clone_fallback", n), &data, |b, data| {
            b.iter(|| ex09_wo::copy_generic(std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("manual_memcpy", n), &data, |b, data| {
            b.iter(|| ex09_wo::copy_memcpy_i32(std::hint::black_box(data)))
        });
    }
    g.finish();
}

// ── Ex10: Dedup ──────────────────────────────────────────────────────────
fn bench_ex10(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex10_dedup_u8");
    for &n in &SIZES {
        let data = make_sorted_u8(n);
        g.throughput(Throughput::Elements(n as u64));
        let dd = ex10::Deduplicator;
        g.bench_with_input(BenchmarkId::new("spec", n), &data, |b, data| {
            b.iter(|| ex10::dedup_u8(&dd, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("generic", n), &data, |b, data| {
            b.iter(|| ex10::dedup_generic(&dd, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &data, |b, data| {
            let e = ex10_wo::DedupEnum::U8(data);
            b.iter(|| ex10_wo::dedup_enum_u8(&e))
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &data, |b, data| {
            b.iter(|| ex10_wo::dedup_runtime_u8(std::hint::black_box(data)))
        });
    }
    g.finish();
}

// ── Ex11: Map double ─────────────────────────────────────────────────────
fn bench_ex11(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex11_map_i32");
    for &n in &SIZES {
        let data = make_i32(n);
        g.throughput(Throughput::Elements(n as u64));
        let m = ex11::Mapper;
        g.bench_with_input(BenchmarkId::new("spec", n), &data, |b, data| {
            b.iter(|| ex11::map_i32(&m, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("generic", n), &data, |b, data| {
            b.iter(|| ex11::map_generic(&m, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &data, |b, data| {
            let e = ex11_wo::MapEnum::I32(data);
            b.iter(|| ex11_wo::map_enum_i32(&e))
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &data, |b, data| {
            b.iter(|| ex11_wo::map_runtime_i32(std::hint::black_box(data)))
        });
    }
    g.finish();
}

// ── Ex12: Sort ───────────────────────────────────────────────────────────
fn bench_ex12(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex12_sort_u8");
    for &n in &SIZES {
        let base = make_u8(n);
        g.throughput(Throughput::Elements(n as u64));
        let sr = ex12::Sorter;
        g.bench_with_input(BenchmarkId::new("spec", n), &base, |b, base| {
            b.iter(|| { let mut d = base.clone(); ex12::sort_u8(&sr, std::hint::black_box(&mut d)); d })
        });
        g.bench_with_input(BenchmarkId::new("generic", n), &base, |b, base| {
            b.iter(|| { let mut d = base.clone(); ex12::sort_generic(&sr, std::hint::black_box(&mut d)); d })
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &base, |b, base| {
            b.iter(|| { let mut d = base.clone(); ex12_wo::sort_runtime_u8(std::hint::black_box(&mut d)); d })
        });
        g.bench_with_input(BenchmarkId::new("generic_stdlib", n), &base, |b, base| {
            b.iter(|| { let mut d = base.clone(); ex12_wo::sort_generic_u8(std::hint::black_box(&mut d)); d })
        });
        g.bench_with_input(BenchmarkId::new("manual_counting", n), &base, |b, base| {
            b.iter(|| { let mut d = base.clone(); ex12_wo::sort_counting_u8(std::hint::black_box(&mut d)); d })
        });
    }
    g.finish();
}

// ── Ex13: Format ─────────────────────────────────────────────────────────
fn bench_ex13(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex13_fmt_i32");
    let counts = [256, 1024, 4096];
    for &n in &counts {
        let data = make_i32(n);
        g.throughput(Throughput::Elements(n as u64));
        let f = ex13::Formatter;
        g.bench_with_input(BenchmarkId::new("spec", n), &data, |b, data| {
            b.iter(|| ex13::fmt_i32(&f, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("generic", n), &data, |b, data| {
            b.iter(|| ex13::fmt_generic(&f, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &data, |b, data| {
            let e = ex13_wo::FmtEnum::I32(data);
            b.iter(|| ex13_wo::fmt_enum_all(&e))
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &data, |b, data| {
            b.iter(|| ex13_wo::fmt_runtime(std::hint::black_box(data)))
        });
    }
    g.finish();
}

// ── Ex14: MinMax ─────────────────────────────────────────────────────────
fn bench_ex14(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex14_minmax_i32");
    for &n in &SIZES {
        let data = make_i32(n);
        g.throughput(Throughput::Elements(n as u64));
        let mm = ex14::MinMaxer;
        g.bench_with_input(BenchmarkId::new("spec", n), &data, |b, data| {
            b.iter(|| ex14::minmax_i32(&mm, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &data, |b, data| {
            let e = ex14_wo::MinMaxEnum::I32(data);
            b.iter(|| ex14_wo::minmax_enum_i32(&e))
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &data, |b, data| {
            b.iter(|| ex14_wo::minmax_runtime_i32(std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("generic", n), &data, |b, data| {
            b.iter(|| ex14_wo::minmax_generic_i32(std::hint::black_box(data)))
        });
    }
    g.finish();
}

// ── Ex15: Contains ───────────────────────────────────────────────────────
fn bench_ex15(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex15_contains_u8");
    for &n in &SIZES {
        let data = make_u8(n);
        let needle: u8 = 200;
        g.throughput(Throughput::Elements(n as u64));
        let sc = ex15::Searcher;
        g.bench_with_input(BenchmarkId::new("spec", n), &data, |b, data| {
            b.iter(|| ex15::contains_u8(&sc, std::hint::black_box(data), &needle))
        });
        g.bench_with_input(BenchmarkId::new("generic", n), &data, |b, data| {
            b.iter(|| ex15::contains_generic(&sc, std::hint::black_box(data), &needle))
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &data, |b, data| {
            let e = ex15_wo::ContainsEnum::U8(data, needle);
            b.iter(|| ex15_wo::contains_enum(&e))
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &data, |b, data| {
            b.iter(|| ex15_wo::contains_runtime_u8(std::hint::black_box(data), &needle))
        });
        g.bench_with_input(BenchmarkId::new("dyn", n), &data, |b, data| {
            let s = ex15_wo::DynU8Search { haystack: data, needle };
            b.iter(|| s.contains_dyn())
        });
    }
    g.finish();
}

// ── Ex16: Fill ───────────────────────────────────────────────────────────
fn bench_ex16(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex16_fill_u8");
    for &n in &SIZES {
        g.throughput(Throughput::Elements(n as u64));
        let fl = ex16::Filler;
        g.bench_with_input(BenchmarkId::new("spec", n), &n, |b, &n| {
            let mut buf = vec![0u8; n];
            b.iter(|| ex16::fill_u8(&fl, std::hint::black_box(&mut buf), 0x42))
        });
        g.bench_with_input(BenchmarkId::new("generic", n), &n, |b, &n| {
            let mut buf = vec![0u8; n];
            b.iter(|| ex16::fill_generic(&fl, std::hint::black_box(&mut buf), 0x42))
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &n, |b, &n| {
            let mut buf = vec![0u8; n];
            b.iter(|| ex16_wo::fill_runtime_u8(std::hint::black_box(&mut buf), 0x42))
        });
        g.bench_with_input(BenchmarkId::new("manual_memset", n), &n, |b, &n| {
            let mut buf = vec![0u8; n];
            b.iter(|| ex16_wo::fill_memset_u8(std::hint::black_box(&mut buf), 0x42))
        });
    }
    g.finish();
}

// ── Ex17: Clamp ──────────────────────────────────────────────────────────
fn bench_ex17(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex17_clamp_i32");
    for &n in &SIZES {
        let data = make_i32(n);
        g.throughput(Throughput::Elements(n as u64));
        let cl = ex17::Clamper;
        g.bench_with_input(BenchmarkId::new("spec", n), &data, |b, data| {
            b.iter(|| ex17::clamp_i32(&cl, std::hint::black_box(data), 100, 900))
        });
        g.bench_with_input(BenchmarkId::new("generic", n), &data, |b, data| {
            b.iter(|| ex17::clamp_generic(&cl, std::hint::black_box(data), 100, 900))
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &data, |b, data| {
            let e = ex17_wo::ClampEnum::I32(data, 100, 900);
            b.iter(|| ex17_wo::clamp_enum_i32(&e))
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &data, |b, data| {
            b.iter(|| ex17_wo::clamp_runtime_i32(std::hint::black_box(data), 100, 900))
        });
    }
    g.finish();
}

// ── Ex18: FMA ────────────────────────────────────────────────────────────
fn bench_ex18(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex18_fma_f64");
    for &n in &SIZES {
        let data = make_f64(n);
        g.throughput(Throughput::Elements(n as u64));
        let fc = ex18::FmaComputer;
        g.bench_with_input(BenchmarkId::new("spec", n), &data, |b, data| {
            b.iter(|| ex18::fma_f64(&fc, std::hint::black_box(data), 2.5, 1.0))
        });
        g.bench_with_input(BenchmarkId::new("generic", n), &data, |b, data| {
            b.iter(|| ex18::fma_generic(&fc, std::hint::black_box(data), 2.5, 1.0))
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &data, |b, data| {
            let e = ex18_wo::FmaEnum::F64(data, 2.5, 1.0);
            b.iter(|| ex18_wo::fma_enum_f64(&e))
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &data, |b, data| {
            b.iter(|| ex18_wo::fma_runtime_f64(std::hint::black_box(data), 2.5, 1.0))
        });
    }
    g.finish();
}

// ── Ex19: From bytes ─────────────────────────────────────────────────────
fn bench_ex19(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex19_frombytes_u8");
    for &n in &SIZES {
        let data = make_u8(n);
        g.throughput(Throughput::Elements(n as u64));
        let cv = ex19::Converter;
        g.bench_with_input(BenchmarkId::new("spec", n), &data, |b, data| {
            b.iter(|| ex19::from_bytes_u8(&cv, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("generic", n), &data, |b, data| {
            b.iter(|| ex19::from_bytes_generic::<u8>(&cv, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &data, |b, data| {
            let e = ex19_wo::FromBytesEnum::U8(data);
            b.iter(|| ex19_wo::frombytes_enum_u8(&e))
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &data, |b, data| {
            b.iter(|| ex19_wo::from_bytes_runtime_u8(std::hint::black_box(data)))
        });
    }
    g.finish();
}

// ── Ex20: Dot product ────────────────────────────────────────────────────
fn bench_ex20(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex20_dot_f64");
    for &n in &SIZES {
        let a = make_f64(n);
        let b_data = make_f64(n);
        g.throughput(Throughput::Elements(n as u64));
        let dp = ex20::DotProduct;
        g.bench_with_input(BenchmarkId::new("spec", n), &n, |bench, _| {
            bench.iter(|| ex20::dot_f64(&dp, std::hint::black_box(&a), std::hint::black_box(&b_data)))
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &n, |bench, _| {
            let e = ex20_wo::DotEnum::F64(&a, &b_data);
            bench.iter(|| ex20_wo::dot_enum_f64(&e))
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &n, |bench, _| {
            bench.iter(|| ex20_wo::dot_runtime_f64(std::hint::black_box(&a), std::hint::black_box(&b_data)))
        });
        g.bench_with_input(BenchmarkId::new("generic", n), &n, |bench, _| {
            bench.iter(|| ex20_wo::dot_generic_f64(std::hint::black_box(&a), std::hint::black_box(&b_data)))
        });
    }
    g.finish();
}

// ── Ex21: Lifetime specialization ────────────────────────────────────────
fn bench_ex21(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex21_lifetime_static");
    // Build a Vec of &'static str
    let static_strs: Vec<&'static str> = vec!["hello"; 65536];
    for &n in &SIZES {
        let items = &static_strs[..n];
        g.throughput(Throughput::Elements(n as u64));
        let p = ex21::Parser;
        g.bench_with_input(BenchmarkId::new("spec_static", n), &items, |b, items| {
            b.iter(|| ex21::sum_parse_static(&p, std::hint::black_box(items)))
        });
        g.bench_with_input(BenchmarkId::new("spec_borrowed", n), &items, |b, items| {
            b.iter(|| ex21::sum_parse_borrowed(&p, std::hint::black_box(items)))
        });
        g.bench_with_input(BenchmarkId::new("spec_generic", n), &items, |b, items| {
            b.iter(|| ex21::sum_parse_generic(&p, std::hint::black_box(items)))
        });
        g.bench_with_input(BenchmarkId::new("enum_static", n), &items, |b, items| {
            b.iter(|| ex21_wo::sum_parse_enum_static(std::hint::black_box(items)))
        });
        g.bench_with_input(BenchmarkId::new("dyn_static", n), &items, |b, items| {
            b.iter(|| ex21_wo::sum_parse_dyn_static(std::hint::black_box(items)))
        });
        g.bench_with_input(BenchmarkId::new("generic", n), &items, |b, items| {
            b.iter(|| ex21_wo::sum_parse_generic(std::hint::black_box(items)))
        });
    }
    g.finish();
}

// ── Ex22: HRTB — higher-order function application ───────────────────────
fn bench_ex22(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex22_hrtb_apply");
    fn double_i32(x: &i32) -> i32 { x * 2 }
    for &n in &SIZES {
        let data = make_i32(n);
        g.throughput(Throughput::Elements(n as u64));
        let a = ex22::Applicator;
        g.bench_with_input(BenchmarkId::new("spec_hrtb", n), &data, |b, data| {
            b.iter(|| ex22::apply_i32_fn(&a, double_i32, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("spec_generic", n), &data, |b, data| {
            b.iter(|| ex22::apply_generic(&a, double_i32, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &data, |b, data| {
            let e = ex22_wo::ApplyEnum::I32(double_i32, data);
            b.iter(|| ex22_wo::apply_enum_i32(&e))
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &data, |b, data| {
            b.iter(|| ex22_wo::apply_runtime_i32(double_i32, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("generic", n), &data, |b, data| {
            b.iter(|| ex22_wo::apply_generic(double_i32, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("dyn", n), &data, |b, data| {
            let d = ex22_wo::DynI32Apply { f: double_i32, data };
            b.iter(|| d.apply_dyn())
        });
    }
    g.finish();
}

// ── Ex23: Multi-generic repeating ────────────────────────────────────────
fn bench_ex23(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex23_multi_generic");
    for &n in &SIZES {
        let xs = make_u8(n);
        let ys: Vec<Vec<u8>> = (0..n).map(|i| vec![(i & 0xFF) as u8; 4]).collect();
        g.throughput(Throughput::Elements(n as u64));
        let comb = ex23::Combiner;
        g.bench_with_input(BenchmarkId::new("spec_u8_vecu8", n), &n, |b, _| {
            b.iter(|| ex23::combine_u8_vecu8(&comb, std::hint::black_box(&xs), std::hint::black_box(&ys)))
        });
        g.bench_with_input(BenchmarkId::new("spec_generic", n), &n, |b, _| {
            b.iter(|| ex23::combine_generic(&comb, std::hint::black_box(&xs), std::hint::black_box(&ys)))
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &n, |b, _| {
            let e = ex23_wo::CombineEnum::U8VecU8(&xs, &ys);
            b.iter(|| ex23_wo::combine_enum(&e))
        });
        g.bench_with_input(BenchmarkId::new("manual", n), &n, |b, _| {
            b.iter(|| ex23_wo::combine_u8_vecu8(std::hint::black_box(&xs), std::hint::black_box(&ys)))
        });
        g.bench_with_input(BenchmarkId::new("generic", n), &n, |b, _| {
            b.iter(|| ex23_wo::combine_generic(std::hint::black_box(&xs), std::hint::black_box(&ys)))
        });
    }
    g.finish();
}

// ── Ex24: Wildcard & complex type patterns ───────────────────────────────
fn bench_ex24(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex24_wildcard_types");
    for &n in &SIZES {
        let items: Vec<Vec<i32>> = (0..n).map(|i| vec![i as i32; 4]).collect();
        g.throughput(Throughput::Elements(n as u64));
        let m = ex24::Measurer;
        g.bench_with_input(BenchmarkId::new("spec_vec_wildcard", n), &n, |b, _| {
            b.iter(|| ex24::sum_measure_vec_i32(&m, std::hint::black_box(&items)))
        });
        g.bench_with_input(BenchmarkId::new("spec_generic", n), &n, |b, _| {
            b.iter(|| ex24::sum_measure_generic(&m, std::hint::black_box(&items)))
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &n, |b, _| {
            b.iter(|| ex24_wo::sum_measure_enum_vec_i32(std::hint::black_box(&items)))
        });
        g.bench_with_input(BenchmarkId::new("manual", n), &n, |b, _| {
            b.iter(|| ex24_wo::sum_measure_manual(std::hint::black_box(&items)))
        });
        g.bench_with_input(BenchmarkId::new("generic", n), &n, |b, _| {
            b.iter(|| ex24_wo::sum_measure_generic(std::hint::black_box(&items)))
        });
    }
    g.finish();
}

// ── Ex25: Conditional combinators (any/all/not) ──────────────────────────
fn bench_ex25(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex25_conditional");
    for &n in &SIZES {
        let data_i32 = make_i32(n);
        let data_u8 = make_u8(n);
        g.throughput(Throughput::Elements(n as u64));
        let p = ex25::Processor;
        g.bench_with_input(BenchmarkId::new("spec_any_i32", n), &data_i32, |b, data| {
            b.iter(|| ex25::process_i32(&p, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("spec_not_u8", n), &data_u8, |b, data| {
            b.iter(|| ex25::process_u8(&p, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("spec_generic", n), &data_i32, |b, data| {
            b.iter(|| ex25::process_generic(&p, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("enum_i32", n), &data_i32, |b, data| {
            let e = ex25_wo::ProcessEnum::I32(data);
            b.iter(|| ex25_wo::process_enum(&e))
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &data_i32, |b, data| {
            b.iter(|| ex25_wo::process_runtime_i32(std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("manual_i32", n), &data_i32, |b, data| {
            b.iter(|| ex25_wo::process_manual_i32(std::hint::black_box(data)))
        });
    }
    g.finish();
}

// ── Ex26: Fn trait bound specialization ──────────────────────────────────
fn bench_ex26(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex26_fn_trait_bounds");
    fn add_one(x: i32) -> i32 { x + 1 }
    fn deref_add(x: &i32) -> i32 { *x + 1 }
    let fns_i32: Vec<fn(i32) -> i32> = vec![add_one; 65536];
    let fns_ref: Vec<fn(&i32) -> i32> = vec![deref_add; 65536];
    for &n in &SIZES {
        g.throughput(Throughput::Elements(n as u64));
        let d = ex26::Dispatcher;
        let slice_i32 = &fns_i32[..n];
        let slice_ref = &fns_ref[..n];
        g.bench_with_input(BenchmarkId::new("spec_fn_i32", n), &n, |b, _| {
            b.iter(|| ex26::sum_dispatch_fn_i32(&d, std::hint::black_box(slice_i32)))
        });
        g.bench_with_input(BenchmarkId::new("spec_fn_ref_i32", n), &n, |b, _| {
            b.iter(|| ex26::sum_dispatch_fn_ref_i32(&d, std::hint::black_box(slice_ref)))
        });
        g.bench_with_input(BenchmarkId::new("spec_generic", n), &n, |b, _| {
            b.iter(|| ex26::sum_dispatch_generic(&d, std::hint::black_box(slice_i32)))
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &n, |b, _| {
            b.iter(|| ex26_wo::sum_dispatch_enum(std::hint::black_box(slice_i32)))
        });
        g.bench_with_input(BenchmarkId::new("manual", n), &n, |b, _| {
            b.iter(|| ex26_wo::sum_dispatch_manual(std::hint::black_box(slice_i32)))
        });
        g.bench_with_input(BenchmarkId::new("dyn", n), &n, |b, _| {
            b.iter(|| ex26_wo::sum_dispatch_dyn(std::hint::black_box(slice_i32)))
        });
    }
    g.finish();
}

// ── Ex27: Multiple self types ────────────────────────────────────────────
fn bench_ex27(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex27_multi_self");
    for &n in &SIZES {
        let data = make_u8(n);
        g.throughput(Throughput::Elements(n as u64));
        let a = ex27::TypeA;
        let b_type = ex27::TypeB;
        g.bench_with_input(BenchmarkId::new("spec_typeA_u8", n), &data, |b, data| {
            b.iter(|| ex27::sum_transform_a_u8(&a, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("spec_typeB_u8", n), &data, |b, data| {
            b.iter(|| ex27::sum_transform_b_u8(&b_type, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("spec_generic", n), &data, |b, data| {
            b.iter(|| ex27::sum_transform_generic_a(&a, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &data, |b, data| {
            b.iter(|| ex27_wo::sum_transform_enum_u8(std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("manual", n), &data, |b, data| {
            b.iter(|| ex27_wo::sum_transform_manual_u8(std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("dyn", n), &data, |b, data| {
            b.iter(|| ex27_wo::sum_transform_dyn(std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("generic", n), &data, |b, data| {
            b.iter(|| ex27_wo::sum_transform_generic(std::hint::black_box(data)))
        });
    }
    g.finish();
}

// ── Ex28: 'static lifetime bound ─────────────────────────────────────────
fn bench_ex28(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex28_static_bound");
    for &n in &SIZES {
        let data = make_i32(n);
        g.throughput(Throughput::Elements(n as u64));
        let h = ex28::Hasher;
        g.bench_with_input(BenchmarkId::new("spec", n), &data, |b, data| {
            b.iter(|| ex28::sum_hash_i32(&h, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("generic", n), &data, |b, data| {
            b.iter(|| ex28::sum_hash_generic(&h, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &data, |b, data| {
            b.iter(|| ex28_wo::sum_hash_enum(std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &data, |b, data| {
            b.iter(|| ex28_wo::sum_hash_runtime(std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("dyn", n), &data, |b, data| {
            b.iter(|| ex28_wo::sum_hash_dyn(std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("manual", n), &data, |b, data| {
            b.iter(|| ex28_wo::sum_hash_manual(std::hint::black_box(data)))
        });
    }
    g.finish();
}

// ── Ex29: &dyn Trait type pattern ────────────────────────────────────────
fn bench_ex29(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex29_dyn_trait_type");
    for &n in &SIZES {
        let data = make_i32(n);
        g.throughput(Throughput::Elements(n as u64));
        let r = ex29::Renderer;
        g.bench_with_input(BenchmarkId::new("spec", n), &data, |b, data| {
            b.iter(|| ex29::sum_render_i32(&r, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("generic", n), &data, |b, data| {
            b.iter(|| ex29::sum_render_generic(&r, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &data, |b, data| {
            b.iter(|| ex29_wo::sum_render_enum(std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &data, |b, data| {
            b.iter(|| ex29_wo::sum_render_runtime(std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("dyn", n), &data, |b, data| {
            b.iter(|| ex29_wo::sum_render_dyn(std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("manual", n), &data, |b, data| {
            b.iter(|| ex29_wo::sum_render_manual(std::hint::black_box(data)))
        });
    }
    g.finish();
}

// ── Ex30: Combined trait bounds (Copy + Eq) ──────────────────────────────
fn bench_ex30(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex30_combined_bounds");
    // Use small chunks so the O(n²) generic doesn't dominate too much
    let small_sizes: [usize; 4] = [16, 64, 128, 256];
    for &n in &small_sizes {
        let data = make_u8(n);
        let chunks: Vec<&[u8]> = data.chunks(n.min(64)).collect();
        g.throughput(Throughput::Elements(n as u64));
        let d = ex30::Deduper;
        g.bench_with_input(BenchmarkId::new("spec_u8", n), &n, |b, _| {
            b.iter(|| ex30::sum_dedup_u8(&d, std::hint::black_box(&chunks)))
        });
        g.bench_with_input(BenchmarkId::new("generic", n), &n, |b, _| {
            b.iter(|| ex30::sum_dedup_generic(&d, std::hint::black_box(&chunks)))
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &n, |b, _| {
            b.iter(|| ex30_wo::sum_dedup_enum_u8(std::hint::black_box(&chunks)))
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &n, |b, _| {
            b.iter(|| ex30_wo::sum_dedup_runtime_u8(std::hint::black_box(&chunks)))
        });
        g.bench_with_input(BenchmarkId::new("manual", n), &n, |b, _| {
            b.iter(|| ex30_wo::sum_dedup_manual_u8(std::hint::black_box(&chunks)))
        });
    }
    g.finish();
}

// ── Ex31: Many specializations (10 types) ────────────────────────────────
fn bench_ex31(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex31_many_specs");
    for &n in &SIZES {
        let data_i32 = make_i32(n);
        let chunks_i32: Vec<&[i32]> = data_i32.chunks(n.min(1024)).collect();
        g.throughput(Throughput::Elements(n as u64));
        let e = ex31::Encoder;
        g.bench_with_input(BenchmarkId::new("spec_i32", n), &n, |b, _| {
            b.iter(|| ex31::sum_encode_i32(&e, std::hint::black_box(&chunks_i32)))
        });
        g.bench_with_input(BenchmarkId::new("generic", n), &n, |b, _| {
            b.iter(|| ex31::sum_encode_generic(&e, std::hint::black_box(&chunks_i32)))
        });
        g.bench_with_input(BenchmarkId::new("enum", n), &n, |b, _| {
            b.iter(|| ex31_wo::sum_encode_enum_i32(std::hint::black_box(&chunks_i32)))
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &n, |b, _| {
            b.iter(|| ex31_wo::sum_encode_runtime_i32(std::hint::black_box(&chunks_i32)))
        });
        g.bench_with_input(BenchmarkId::new("manual", n), &n, |b, _| {
            b.iter(|| ex31_wo::sum_encode_manual_i32(std::hint::black_box(&chunks_i32)))
        });
        g.bench_with_input(BenchmarkId::new("dyn", n), &n, |b, _| {
            b.iter(|| ex31_wo::sum_encode_dyn_i32(std::hint::black_box(&chunks_i32)))
        });
    }
    g.finish();
}

// ── Ex32: Tuple type specialization ──────────────────────────────────────
fn bench_ex32(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex32_tuple_types");
    for &n in &SIZES {
        let data_i32: Vec<(i32, i32)> = (0..n).map(|i| (i as i32, (i * 2) as i32)).collect();
        let data_f64: Vec<(f64, f64)> = (0..n).map(|i| (i as f64, (i * 2) as f64)).collect();
        g.throughput(Throughput::Elements(n as u64));
        let ops = ex32::PointOps;
        g.bench_with_input(BenchmarkId::new("spec_i32", n), &data_i32, |b, data| {
            b.iter(|| ex32::squared_sum_i32(&ops, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("spec_f64", n), &data_f64, |b, data| {
            b.iter(|| ex32::squared_sum_f64(&ops, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("generic", n), &data_i32, |b, data| {
            b.iter(|| ex32::squared_sum_generic(&ops, std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("enum_i32", n), &data_i32, |b, data| {
            let e = ex32_wo::PointEnum::I32(data);
            b.iter(|| ex32_wo::squared_sum_enum(&e))
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &data_i32, |b, data| {
            b.iter(|| ex32_wo::squared_sum_runtime_i32(std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("manual", n), &data_i32, |b, data| {
            b.iter(|| ex32_wo::squared_sum_manual_i32(std::hint::black_box(data)))
        });
        g.bench_with_input(BenchmarkId::new("dyn", n), &data_i32, |b, data| {
            let d = ex32_wo::DynI32Points(data);
            let dd: &dyn ex32_wo::DynSquaredSum = &d;
            b.iter(|| dd.squared_sum_dyn())
        });
    }
    g.finish();
}

// ── Ex33: Nested generic types ───────────────────────────────────────────
fn bench_ex33(c: &mut Criterion) {
    let mut g = c.benchmark_group("ex33_nested_generics");
    for &n in &SIZES {
        let data_vvi: Vec<Vec<Vec<u8>>> = (0..n.min(1024))
            .map(|i| vec![vec![(i & 0xFF) as u8; 4]; 2])
            .collect();
        let data_vi: Vec<Vec<i32>> = (0..n.min(1024))
            .map(|i| vec![i as i32; 4])
            .collect();
        let elem_count = n.min(1024);
        g.throughput(Throughput::Elements(elem_count as u64));
        let f = ex33::Flattener;
        // Keep IDs unique across the whole group, even when data size is capped.
        g.bench_with_input(BenchmarkId::new("spec_vec_vec_u8", n), &elem_count, |b, _| {
            b.iter(|| ex33::flat_len_vec_vec_u8(&f, std::hint::black_box(&data_vvi)))
        });
        g.bench_with_input(BenchmarkId::new("spec_vec_i32", n), &elem_count, |b, _| {
            b.iter(|| ex33::flat_len_vec_i32(&f, std::hint::black_box(&data_vi)))
        });
        g.bench_with_input(BenchmarkId::new("generic", n), &elem_count, |b, _| {
            b.iter(|| ex33::flat_len_generic(&f, std::hint::black_box(&data_vvi)))
        });
        g.bench_with_input(BenchmarkId::new("enum_vec_vec_u8", n), &elem_count, |b, _| {
            let e = ex33_wo::FlatEnum::VecVecU8(&data_vvi);
            b.iter(|| ex33_wo::flat_enum(&e))
        });
        g.bench_with_input(BenchmarkId::new("runtime_typeid", n), &elem_count, |b, _| {
            b.iter(|| ex33_wo::flat_runtime_vec_vec_u8(std::hint::black_box(&data_vvi)))
        });
        g.bench_with_input(BenchmarkId::new("manual", n), &elem_count, |b, _| {
            b.iter(|| ex33_wo::flat_manual_vec_vec_u8(std::hint::black_box(&data_vvi)))
        });
        g.bench_with_input(BenchmarkId::new("dyn", n), &elem_count, |b, _| {
            let d = ex33_wo::DynVecVecU8(&data_vvi);
            let dd: &dyn ex33_wo::DynFlat = &d;
            b.iter(|| dd.flat_dyn())
        });
    }
    g.finish();
}

criterion_group!(
    benches,
    bench_ex01, bench_ex02, bench_ex03, bench_ex04, bench_ex05,
    bench_ex06, bench_ex07, bench_ex08, bench_ex09, bench_ex10,
    bench_ex11, bench_ex12, bench_ex13, bench_ex14, bench_ex15,
    bench_ex16, bench_ex17, bench_ex18, bench_ex19, bench_ex20,
    bench_ex21, bench_ex22, bench_ex23, bench_ex24, bench_ex25,
    bench_ex26, bench_ex27, bench_ex28, bench_ex29, bench_ex30,
    bench_ex31, bench_ex32, bench_ex33,
);
criterion_main!(benches);
