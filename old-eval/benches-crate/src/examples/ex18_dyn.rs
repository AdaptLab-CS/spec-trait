// Example 18 — DYN: HRTB — higher-order function application


// --- dyn dispatch ---
pub trait DynApply {
    fn apply_dyn(&self) -> Vec<i32>;
}

pub struct DynI32Apply<'a> {
    pub f: fn(&i32) -> i32,
    pub data: &'a [i32],
}
impl DynApply for DynI32Apply<'_> {
    #[inline(never)]
    fn apply_dyn(&self) -> Vec<i32> {
        self.data.iter().map(|x| (self.f)(x)).collect()
    }
}
