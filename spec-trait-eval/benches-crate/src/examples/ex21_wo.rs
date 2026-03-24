// Example 21 — WITHOUT macros: Lifetime specialization (static vs borrowed)
use std::any::TypeId;

// --- enum dispatch ---
pub enum ParseEnum<'a> {
    StaticStr(&'static str),
    BorrowedStr(&'a str),
    StaticBytes(&'static [u8]),
    BorrowedBytes(&'a [u8]),
}

#[inline(never)]
pub fn parse_enum(variant: &ParseEnum<'_>) -> usize {
    match variant {
        ParseEnum::StaticStr(_) => 42,
        ParseEnum::BorrowedStr(_) => 1,
        ParseEnum::StaticBytes(_) => 42,
        ParseEnum::BorrowedBytes(_) => 1,
    }
}

// --- runtime check ---
#[inline(never)]
pub fn parse_runtime_static_str(data: &'static str) -> usize {
    if TypeId::of::<&'static str>() == TypeId::of::<&'static str>() {
        42
    } else {
        data.len()
    }
}

#[inline(never)]
pub fn parse_runtime_borrowed_str(data: &str) -> usize {
    // Can't distinguish lifetimes at runtime; always generic path
    data.len()
}

// --- generic fallback ---
#[inline(never)]
pub fn parse_generic<T>(_data: T) -> usize {
    0
}

// --- dyn dispatch ---
pub trait DynParse {
    fn parse_dyn(&self) -> usize;
}

pub struct DynStaticStr(pub &'static str);
impl DynParse for DynStaticStr {
    #[inline(never)]
    fn parse_dyn(&self) -> usize { 42 }
}

pub struct DynBorrowedStr<'a>(pub &'a str);
impl DynParse for DynBorrowedStr<'_> {
    #[inline(never)]
    fn parse_dyn(&self) -> usize { 1 }
}

// Benchmarkable bulk functions
#[inline(never)]
pub fn sum_parse_enum_static(items: &[&'static str]) -> usize {
    let mut total = 0usize;
    for &s in items {
        let e = ParseEnum::StaticStr(s);
        total = total.wrapping_add(parse_enum(&e));
    }
    total
}

#[inline(never)]
pub fn sum_parse_enum_borrowed<'a>(items: &[&'a str]) -> usize {
    let mut total = 0usize;
    for &s in items {
        let e = ParseEnum::BorrowedStr(s);
        total = total.wrapping_add(parse_enum(&e));
    }
    total
}

#[inline(never)]
pub fn sum_parse_generic(items: &[&str]) -> usize {
    let mut total = 0usize;
    for &s in items {
        total = total.wrapping_add(parse_generic(s));
    }
    total
}

#[inline(never)]
pub fn sum_parse_dyn_static(items: &[&'static str]) -> usize {
    let mut total = 0usize;
    for &s in items {
        let d = DynStaticStr(s);
        total = total.wrapping_add(d.parse_dyn());
    }
    total
}
