// Example 31 — DYN: not() predicate – type exclusion


// --- dyn dispatch ---
pub trait DynCompute {
    fn compute_dyn(&self) -> usize;
}

pub struct DynComputeI32<'a>(pub &'a [i32]);
impl DynCompute for DynComputeI32<'_> {
    #[inline(never)]
    fn compute_dyn(&self) -> usize {
        self.0.len() * std::mem::size_of::<i32>()
    }
}

pub struct DynComputeString<'a>(pub &'a [String]);
impl DynCompute for DynComputeString<'_> {
    #[inline(never)]
    fn compute_dyn(&self) -> usize {
        let mut total = 0usize;
        for s in self.0 {
            total += s.len();
        }
        total
    }
}

