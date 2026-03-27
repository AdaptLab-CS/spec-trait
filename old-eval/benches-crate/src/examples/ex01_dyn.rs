// Example 01 — DYN: Sum


// --- dyn dispatch ---
pub trait DynSum {
    fn sum_dyn(&self) -> f64;
}
impl DynSum for Vec<i32> {
    #[inline(never)]
    fn sum_dyn(&self) -> f64 {
        self.iter().copied().sum::<i32>() as f64
    }
}
impl DynSum for Vec<f64> {
    #[inline(never)]
    fn sum_dyn(&self) -> f64 {
        self.iter().copied().sum()
    }
}

pub fn sum_dyn(data: &dyn DynSum) -> f64 {
    data.sum_dyn()
}

