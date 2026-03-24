// Example 1 — WITHOUT macros: Saturating byte doubling
// Uses runtime TypeId check + dyn dispatch + enum dispatch as alternatives.
use std::any::TypeId;

pub trait DoubleWo {
    fn double(&self) -> Box<dyn DoubleWo>;
}

// --- Approach A: dyn dispatch ---
pub trait DynDouble {
    fn double_dyn(&self, data: &[u8]) -> Vec<u8>;
}

pub struct DynU8Doubler;
impl DynDouble for DynU8Doubler {
    #[inline(never)]
    fn double_dyn(&self, data: &[u8]) -> Vec<u8> {
        data.iter().map(|b| b.saturating_mul(2)).collect()
    }
}

pub struct DynI32Doubler;
pub trait DynDoubleI32 {
    fn double_dyn(&self, data: &[i32]) -> Vec<i32>;
}
impl DynDoubleI32 for DynI32Doubler {
    #[inline(never)]
    fn double_dyn(&self, data: &[i32]) -> Vec<i32> {
        data.iter().map(|v| v.wrapping_mul(2)).collect()
    }
}

// --- Approach B: enum dispatch ---
pub enum DoubleEnum {
    U8,
    I32,
    Generic,
}

#[inline(never)]
pub fn double_enum_u8(variant: &DoubleEnum, data: &[u8]) -> Vec<u8> {
    match variant {
        DoubleEnum::U8 => data.iter().map(|b| b.saturating_mul(2)).collect(),
        _ => data.to_vec(),
    }
}

#[inline(never)]
pub fn double_enum_i32(variant: &DoubleEnum, data: &[i32]) -> Vec<i32> {
    match variant {
        DoubleEnum::I32 => data.iter().map(|v| v.wrapping_mul(2)).collect(),
        _ => data.to_vec(),
    }
}

// --- Approach C: runtime TypeId ---
#[inline(never)]
pub fn double_runtime_u8(data: &[u8]) -> Vec<u8> {
    if TypeId::of::<u8>() == TypeId::of::<u8>() {
        data.iter().map(|b| b.saturating_mul(2)).collect()
    } else {
        data.to_vec()
    }
}

#[inline(never)]
pub fn double_runtime_i32(data: &[i32]) -> Vec<i32> {
    if TypeId::of::<i32>() == TypeId::of::<i32>() {
        data.iter().map(|v| v.wrapping_mul(2)).collect()
    } else {
        data.to_vec()
    }
}

// --- Approach D: generic fallback (no specialization) ---
#[inline(never)]
pub fn double_generic_u8(data: &[u8]) -> Vec<u8> {
    data.to_vec()
}

#[inline(never)]
pub fn double_generic_i32(data: &[i32]) -> Vec<i32> {
    data.to_vec()
}
