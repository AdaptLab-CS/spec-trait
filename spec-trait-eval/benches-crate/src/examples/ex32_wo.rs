// Example 32 — WITHOUT macros: Tuple type specialization
use std::any::TypeId;

// --- Approach A: enum dispatch ---
pub enum PointEnum<'a> {
    I32(&'a [(i32, i32)]),
    F64(&'a [(f64, f64)]),
    F32(&'a [(f32, f32)]),
}

#[inline(never)]
pub fn squared_sum_enum(variant: &PointEnum) -> f64 {
    match variant {
        PointEnum::I32(data) => {
            let mut sum = 0i64;
            for &(x, y) in *data {
                sum += (x as i64) * (x as i64) + (y as i64) * (y as i64);
            }
            sum as f64
        }
        PointEnum::F64(data) => {
            let mut sum = 0.0f64;
            for &(x, y) in *data {
                sum += x * x + y * y;
            }
            sum
        }
        PointEnum::F32(data) => {
            let mut sum = 0.0f32;
            for &(x, y) in *data {
                sum += x * x + y * y;
            }
            sum as f64
        }
    }
}

// --- Approach B: runtime TypeId ---
#[inline(never)]
pub fn squared_sum_runtime_i32(data: &[(i32, i32)]) -> f64 {
    if TypeId::of::<(i32, i32)>() == TypeId::of::<(i32, i32)>() {
        let mut sum = 0i64;
        for &(x, y) in data {
            sum += (x as i64) * (x as i64) + (y as i64) * (y as i64);
        }
        sum as f64
    } else {
        data.len() as f64
    }
}

#[inline(never)]
pub fn squared_sum_runtime_f64(data: &[(f64, f64)]) -> f64 {
    if TypeId::of::<(f64, f64)>() == TypeId::of::<(f64, f64)>() {
        let mut sum = 0.0f64;
        for &(x, y) in data {
            sum += x * x + y * y;
        }
        sum
    } else {
        data.len() as f64
    }
}

// --- Approach C: generic fallback ---
#[inline(never)]
pub fn squared_sum_generic_i32(data: &[(i32, i32)]) -> f64 {
    data.len() as f64
}

#[inline(never)]
pub fn squared_sum_generic_f64(data: &[(f64, f64)]) -> f64 {
    data.len() as f64
}

// --- Approach D: manual specialization ---
#[inline(never)]
pub fn squared_sum_manual_i32(data: &[(i32, i32)]) -> f64 {
    let mut sum = 0i64;
    for &(x, y) in data {
        sum += (x as i64) * (x as i64) + (y as i64) * (y as i64);
    }
    sum as f64
}

#[inline(never)]
pub fn squared_sum_manual_f64(data: &[(f64, f64)]) -> f64 {
    let mut sum = 0.0f64;
    for &(x, y) in data {
        sum += x * x + y * y;
    }
    sum
}

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
