// Example 29 — DYN: Nested generic type specialization


// --- Approach E: dyn dispatch ---
pub trait DynFlat {
    fn flat_dyn(&self) -> usize;
}

pub struct DynVecVecU8<'a>(pub &'a [Vec<Vec<u8>>]);
impl DynFlat for DynVecVecU8<'_> {
    #[inline(never)]
    fn flat_dyn(&self) -> usize {
        let mut total = 0usize;
        for outer in self.0 {
            for inner in outer {
                total += inner.len();
            }
        }
        total
    }
}

pub struct DynVecI32<'a>(pub &'a [Vec<i32>]);
impl DynFlat for DynVecI32<'_> {
    #[inline(never)]
    fn flat_dyn(&self) -> usize {
        let mut total = 0usize;
        for outer in self.0 {
            total += outer.len() * 4;
        }
        total
    }
}
