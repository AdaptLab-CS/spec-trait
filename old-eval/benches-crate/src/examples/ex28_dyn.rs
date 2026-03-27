// Example 28 — DYN: Tuple type specialization


// --- Approach E: dyn dispatch ---
pub trait DynSquaredSum {
    fn squared_sum_dyn(&self) -> f64;
}

pub struct DynI32Points<'a>(pub &'a [(i32, i32)]);
impl DynSquaredSum for DynI32Points<'_> {
    #[inline(never)]
    fn squared_sum_dyn(&self) -> f64 {
        let mut sum = 0i64;
        for &(x, y) in self.0 {
            sum += (x as i64) * (x as i64) + (y as i64) * (y as i64);
        }
        sum as f64
    }
}

pub struct DynF64Points<'a>(pub &'a [(f64, f64)]);
impl DynSquaredSum for DynF64Points<'_> {
    #[inline(never)]
    fn squared_sum_dyn(&self) -> f64 {
        let mut sum = 0.0f64;
        for &(x, y) in self.0 {
            sum += x * x + y * y;
        }
        sum
    }
}
