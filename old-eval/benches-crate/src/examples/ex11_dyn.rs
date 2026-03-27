// Example 11 — DYN: Contains / search


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
