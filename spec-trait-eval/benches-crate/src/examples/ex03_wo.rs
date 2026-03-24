// Example 3 — WITHOUT macros: Sum
use std::any::TypeId;

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

// --- enum dispatch ---
pub enum SumEnum<'a> {
    I32(&'a [i32]),
    F64(&'a [f64]),
    U8(&'a [u8]),
}

#[inline(never)]
pub fn sum_enum(variant: &SumEnum<'_>) -> f64 {
    match variant {
        SumEnum::I32(d) => d.iter().copied().sum::<i32>() as f64,
        SumEnum::F64(d) => d.iter().copied().sum(),
        SumEnum::U8(d) => {
            let mut acc: u8 = 0;
            for &b in *d { acc = acc.wrapping_add(b); }
            acc as f64
        }
    }
}

// --- TypeId runtime check ---
#[inline(never)]
pub fn sum_runtime_i32(data: &[i32]) -> i32 {
    if TypeId::of::<i32>() == TypeId::of::<i32>() {
        data.iter().copied().sum()
    } else {
        0
    }
}

#[inline(never)]
pub fn sum_runtime_f64(data: &[f64]) -> f64 {
    if TypeId::of::<f64>() == TypeId::of::<f64>() {
        data.iter().copied().sum()
    } else {
        0.0
    }
}

// --- generic fallback (iter fold, no type knowledge) ---
#[inline(never)]
pub fn sum_generic_i32(data: &[i32]) -> i32 {
    data.iter().fold(0i32, |a, &b| a.wrapping_add(b))
}

#[inline(never)]
pub fn sum_generic_f64(data: &[f64]) -> f64 {
    data.iter().fold(0.0f64, |a, &b| a + b)
}
