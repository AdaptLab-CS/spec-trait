// Ex01: extend_from_slice — Copy memcpy vs Clone loop
pub mod ex01_spec;
pub mod ex01_naive;
pub mod ex01_enum;
pub mod ex01_runtime_typeid;

// Ex02: to_bytes — Copy raw transmute vs Display formatting
pub mod ex02_spec;
pub mod ex02_naive;
pub mod ex02_enum;
pub mod ex02_runtime_typeid;

// Ex03: contains — memchr SIMD vs linear scan
pub mod ex03_spec;
pub mod ex03_naive;
pub mod ex03_enum;
pub mod ex03_runtime_typeid;

// Ex04: sort — counting sort (u8) vs sort_unstable
pub mod ex04_spec;
pub mod ex04_naive;
pub mod ex04_enum;
pub mod ex04_runtime_typeid;

// Ex05: histogram — array[256] vs HashMap
pub mod ex05_spec;
pub mod ex05_naive;
pub mod ex05_enum;
pub mod ex05_runtime_typeid;

// Ex06: dedup — bitmap[256] vs HashSet
pub mod ex06_spec;
pub mod ex06_naive;
pub mod ex06_enum;
pub mod ex06_runtime_typeid;

// Ex07: hash_key — identity hash (u64) vs SipHash
pub mod ex07_spec;
pub mod ex07_naive;
pub mod ex07_enum;
pub mod ex07_runtime_typeid;

// Ex08: count_value — memchr SIMD vs filter().count()
pub mod ex08_spec;
pub mod ex08_naive;
pub mod ex08_enum;
pub mod ex08_runtime_typeid;
