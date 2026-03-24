// Example 32 — WITH macros: Tuple type specialization
// Specialize point/coordinate operations based on tuple component types.
use spec_trait_macro::{spec, when};

pub struct PointOps;

pub trait DistanceTrait<T> {
    fn squared_sum(&self, data: &[T]) -> f64;
}

impl<T> DistanceTrait<T> for PointOps {
    #[inline(never)]
    fn squared_sum(&self, data: &[T]) -> f64 {
        // Generic: return count as f64
        data.len() as f64
    }
}

#[when(T = (i32, i32))]
impl<T> DistanceTrait<T> for PointOps {
    #[inline(never)]
    fn squared_sum(&self, data: &[T]) -> f64 {
        // Integer tuples: integer arithmetic, cast at end
        let mut sum = 0i64;
        for pt in data {
            let (x, y) = unsafe { *(pt as *const T as *const (i32, i32)) };
            sum += (x as i64) * (x as i64) + (y as i64) * (y as i64);
        }
        sum as f64
    }
}

#[when(T = (f64, f64))]
impl<T> DistanceTrait<T> for PointOps {
    #[inline(never)]
    fn squared_sum(&self, data: &[T]) -> f64 {
        // Float tuples: direct floating-point arithmetic
        let mut sum = 0.0f64;
        for pt in data {
            let (x, y) = unsafe { *(pt as *const T as *const (f64, f64)) };
            sum += x * x + y * y;
        }
        sum
    }
}

#[when(T = (f32, f32))]
impl<T> DistanceTrait<T> for PointOps {
    #[inline(never)]
    fn squared_sum(&self, data: &[T]) -> f64 {
        let mut sum = 0.0f32;
        for pt in data {
            let (x, y) = unsafe { *(pt as *const T as *const (f32, f32)) };
            sum += x * x + y * y;
        }
        sum as f64
    }
}

pub fn squared_sum_i32(ops: &PointOps, data: &[(i32, i32)]) -> f64 {
    spec! { ops.squared_sum(data); PointOps; [(i32, i32)] }
}

pub fn squared_sum_f64(ops: &PointOps, data: &[(f64, f64)]) -> f64 {
    spec! { ops.squared_sum(data); PointOps; [(f64, f64)] }
}

pub fn squared_sum_f32(ops: &PointOps, data: &[(f32, f32)]) -> f64 {
    spec! { ops.squared_sum(data); PointOps; [(f32, f32)] }
}

pub fn squared_sum_generic<T>(ops: &PointOps, data: &[T]) -> f64 {
    DistanceTrait::squared_sum(ops, data)
}
