// Ex03 — dyn dispatch: Contains search
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

pub fn contains_dyn(d: &dyn DynContains) -> bool {
    d.contains_dyn()
}
