use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use spec_trait_eval_benches::data::*;
use spec_trait_eval_benches::examples::*;
use spec_trait_eval_benches::examples::ex15_wo::DynContains;

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

criterion_group!(
    benches,
    bench_ex01, bench_ex02, bench_ex03, bench_ex04, bench_ex05,
    bench_ex06, bench_ex07, bench_ex08, bench_ex09, bench_ex10,
    bench_ex11, bench_ex12, bench_ex13, bench_ex14, bench_ex15,
    bench_ex16, bench_ex17, bench_ex18, bench_ex19, bench_ex20,
);
criterion_main!(benches);
