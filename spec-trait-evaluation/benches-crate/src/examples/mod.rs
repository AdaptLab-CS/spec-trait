// Ex01: extend_from_slice — Copy memcpy vs Clone loop
pub mod ex01_spec;
pub mod ex01_naive;
pub mod ex01_runtime_typeid;

// Ex02: to_bytes — Copy raw transmute vs Display formatting
pub mod ex02_spec;
pub mod ex02_naive;
pub mod ex02_runtime_typeid;

// Ex03: contains — byte range check vs PartialOrd
pub mod ex03_spec;
pub mod ex03_naive;
pub mod ex03_runtime_typeid;

// Ex04: sort — counting sort (u8) vs sort_unstable
pub mod ex04_spec;
pub mod ex04_naive;
pub mod ex04_runtime_typeid;

// Ex05: histogram — direct byte index vs hash bucket
pub mod ex05_spec;
pub mod ex05_naive;
pub mod ex05_runtime_typeid;

// Ex06: dedup — bitmap[256] vs HashSet
pub mod ex06_spec;
pub mod ex06_naive;
pub mod ex06_runtime_typeid;

// Ex07: hash_key — identity hash (u64) vs SipHash
pub mod ex07_spec;
pub mod ex07_naive;
pub mod ex07_runtime_typeid;

// Ex08: count_value — memchr SIMD vs filter().count()
pub mod ex08_spec;
pub mod ex08_naive;
pub mod ex08_runtime_typeid;

// Ex09: format_collect — 'static lifetime bound (#[when(T: 'static)])
pub mod ex09_spec;
pub mod ex09_naive;
pub mod ex09_runtime_typeid;

// Ex10: sum_slice — reference lifetime (#[when(T = &'static _)] vs #[when(all(T = &_, T: 'a))])
pub mod ex10_spec;
pub mod ex10_naive;
pub mod ex10_runtime_typeid;

// Ex11: map_transform — higher-order fn pointer (#[when(F = for<'a> fn(&'a T) -> T)])
pub mod ex11_spec;
pub mod ex11_naive;
pub mod ex11_runtime_typeid;

// Ex12: fold_with — higher-ranked trait bound (#[when(T: for<'a> Fn(&'a i32) -> i32)])
pub mod ex12_spec;
pub mod ex12_naive;
pub mod ex12_runtime_typeid;

// Ex13: flatten_sum — compound all() (#[when(all(U = Vec<T>, T = u8))])
pub mod ex13_spec;
pub mod ex13_naive;
pub mod ex13_runtime_typeid;

// Ex14: encode_val — negation/alternation (#[when(any(T = i32, T = i64))], #[when(not(...))])
pub mod ex14_spec;
pub mod ex14_naive;
pub mod ex14_runtime_typeid;

// Ex15: compute_sum — trait object (#[when(T = &dyn Summable)])
pub mod ex15_spec;
pub mod ex15_naive;
pub mod ex15_runtime_typeid;

// Ex16: total_capacity — wildcard + type alias (#[when(T = Vec<_>)], #[when(T = Vec<MyAlias>)])
pub mod ex16_spec;
pub mod ex16_naive;
pub mod ex16_runtime_typeid;
