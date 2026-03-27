// Example 33 — DYN: Multiple receiver types (3 receivers, same trait)


// --- dyn dispatch ---
pub trait DynCount {
    fn count_dyn(&self) -> usize;
}

pub struct DynVecCounterU8<'a>(pub &'a [u8]);
impl DynCount for DynVecCounterU8<'_> {
    #[inline(never)]
    fn count_dyn(&self) -> usize {
        let mut c = 0usize;
        for &b in self.0 {
            c += (b != 0) as usize;
        }
        c
    }
}

pub struct DynSliceCounterU8<'a>(pub &'a [u8]);
impl DynCount for DynSliceCounterU8<'_> {
    #[inline(never)]
    fn count_dyn(&self) -> usize {
        let mut total = 0usize;
        for &b in self.0 {
            total = total.wrapping_add(b as usize);
        }
        total
    }
}

pub struct DynArrayCounterU8<'a>(pub &'a [u8]);
impl DynCount for DynArrayCounterU8<'_> {
    #[inline(never)]
    fn count_dyn(&self) -> usize {
        let mut acc = 0u8;
        for &b in self.0 {
            acc ^= b;
        }
        acc as usize
    }
}

