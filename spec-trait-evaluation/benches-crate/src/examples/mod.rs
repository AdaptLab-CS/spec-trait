// Ex01: Histogram — max frequency count (u8 array[256] vs HashMap)
pub mod ex01_spec;
pub mod ex01_dyn;
pub mod ex01_enum;
pub mod ex01_generic;
pub mod ex01_runtime_typeid;

// Ex02: Set intersection count (u8 bitmap[256] vs HashSet)
pub mod ex02_spec;
pub mod ex02_dyn;
pub mod ex02_enum;
pub mod ex02_generic;
pub mod ex02_runtime_typeid;

// Ex03: Contains search (u8 specialization via memchr)
pub mod ex03_spec;
pub mod ex03_dyn;
pub mod ex03_enum;
pub mod ex03_generic;
pub mod ex03_runtime_typeid;

// Ex04: Sort (u8 counting sort vs generic Ord sort)
pub mod ex04_spec;
pub mod ex04_enum;
pub mod ex04_generic;
pub mod ex04_runtime_typeid;

// Ex05: Distinct count (u8 bitmap[256] vs HashSet)
pub mod ex05_spec;
pub mod ex05_dyn;
pub mod ex05_enum;
pub mod ex05_generic;
pub mod ex05_runtime_typeid;

// Ex06: Bulk serialize (u8 zero-copy vs generic Display)
pub mod ex06_spec;
pub mod ex06_enum;
pub mod ex06_generic;
pub mod ex06_runtime_typeid;

// Ex07: Dedup — remove all duplicates (u8 bitmap vs HashSet)
pub mod ex07_spec;
pub mod ex07_enum;
pub mod ex07_generic;
pub mod ex07_runtime_typeid;

// Ex08: Count value occurrences (u8 memchr vs linear scan)
pub mod ex08_spec;
pub mod ex08_enum;
pub mod ex08_generic;
pub mod ex08_runtime_typeid;
