// Example 15 — WITHOUT macros: Contains / search
use std::any::TypeId;

// --- enum ---
pub enum ContainsEnum<'a> {
    U8(&'a [u8], u8),
    I32(&'a [i32], i32),
}
#[inline(never)]
pub fn contains_enum(variant: &ContainsEnum<'_>) -> bool {
    match variant {
        ContainsEnum::U8(haystack, needle) => memchr::memchr(*needle, haystack).is_some(),
        ContainsEnum::I32(haystack, needle) => haystack.iter().any(|x| x == needle),
    }
}

// --- runtime TypeId ---
#[inline(never)]
pub fn contains_runtime_u8(haystack: &[u8], needle: &u8) -> bool {
    if TypeId::of::<u8>() == TypeId::of::<u8>() {
        memchr::memchr(*needle, haystack).is_some()
    } else {
        haystack.contains(needle)
    }
}

// --- generic fallback ---
#[inline(never)]
pub fn contains_generic<T: PartialEq>(haystack: &[T], needle: &T) -> bool {
    haystack.iter().any(|x| x == needle)
}

// --- dyn dispatch ---
pub trait DynContains {
    fn contains_dyn(&self) -> bool;
}
pub struct DynU8Search<'a> {
    pub haystack: &'a [u8],
    pub needle: u8,
}
impl DynContains for DynU8Search<'_> {
    #[inline(never)]
    fn contains_dyn(&self) -> bool {
        memchr::memchr(self.needle, self.haystack).is_some()
    }
}
pub struct DynI32Search<'a> {
    pub haystack: &'a [i32],
    pub needle: i32,
}
impl DynContains for DynI32Search<'_> {
    #[inline(never)]
    fn contains_dyn(&self) -> bool {
        self.haystack.iter().any(|x| *x == self.needle)
    }
}
