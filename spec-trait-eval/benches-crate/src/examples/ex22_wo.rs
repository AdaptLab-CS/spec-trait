// Example 22 — WITHOUT macros: HRTB — higher-order function application
use std::any::TypeId;

// --- enum dispatch ---
pub enum ApplyEnum<'a> {
    I32(fn(&i32) -> i32, &'a [i32]),
    F64(fn(&f64) -> f64, &'a [f64]),
}

#[inline(never)]
pub fn apply_enum_i32(variant: &ApplyEnum<'_>) -> Vec<i32> {
    match variant {
        ApplyEnum::I32(f, data) => data.iter().map(|x| f(x)).collect(),
        _ => unreachable!(),
    }
}

#[inline(never)]
pub fn apply_enum_f64(variant: &ApplyEnum<'_>) -> Vec<f64> {
    match variant {
        ApplyEnum::F64(f, data) => data.iter().map(|x| f(x)).collect(),
        _ => unreachable!(),
    }
}

// --- runtime TypeId ---
#[inline(never)]
pub fn apply_runtime_i32(f: fn(&i32) -> i32, data: &[i32]) -> Vec<i32> {
    if TypeId::of::<i32>() == TypeId::of::<i32>() {
        data.iter().map(|x| f(x)).collect()
    } else {
        data.iter().map(|x| f(x)).collect()
    }
}

// --- generic fallback ---
#[inline(never)]
pub fn apply_generic<F: Fn(&i32) -> i32>(f: F, data: &[i32]) -> Vec<i32> {
    data.iter().map(|x| f(x)).collect()
}

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
