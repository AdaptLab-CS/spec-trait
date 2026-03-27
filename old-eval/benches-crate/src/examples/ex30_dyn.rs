// Example 30 — DYN: any() predicate – 4-way integer fast path


// --- dyn dispatch ---
pub trait DynFilterCount {
    fn filter_count_dyn(&self) -> usize;
}

pub struct DynFilterI32<'a>(pub &'a [i32]);
impl DynFilterCount for DynFilterI32<'_> {
    #[inline(never)]
    fn filter_count_dyn(&self) -> usize {
        self.0.len() * std::mem::size_of::<i32>()
    }
}

pub struct DynFilterU8<'a>(pub &'a [u8]);
impl DynFilterCount for DynFilterU8<'_> {
    #[inline(never)]
    fn filter_count_dyn(&self) -> usize {
        let mut count = 0usize;
        for &b in self.0 {
            count += (b != 0) as usize;
        }
        count
    }
}

