// Example 36 — DYN: Triple trait bounds (Clone + Eq + Ord)


// --- dyn dispatch ---
pub trait DynSortedCount {
    fn sorted_unique_count_dyn(&self) -> usize;
}

pub struct DynSortI32<'a>(pub &'a [i32]);
impl DynSortedCount for DynSortI32<'_> {
    #[inline(never)]
    fn sorted_unique_count_dyn(&self) -> usize {
        let mut buf: Vec<i32> = self.0.to_vec();
        buf.sort_unstable();
        buf.dedup();
        buf.len()
    }
}

pub struct DynSortU8<'a>(pub &'a [u8]);
impl DynSortedCount for DynSortU8<'_> {
    #[inline(never)]
    fn sorted_unique_count_dyn(&self) -> usize {
        let mut buf: Vec<u8> = self.0.to_vec();
        buf.sort_unstable();
        buf.dedup();
        buf.len()
    }
}

