// Example 28 — WITHOUT macros: T: 'static lifetime bound
use std::any::TypeId;

// --- Approach A: enum dispatch ---
pub enum HashEnum {
    StaticI32,
    Generic,
}

#[inline(never)]
pub fn hash_enum_i32(variant: &HashEnum, data: &i32) -> u64 {
    match variant {
        HashEnum::StaticI32 => {
            let ptr = data as *const i32 as usize;
            (ptr as u64).wrapping_mul(0x517cc1b727220a95)
        }
        HashEnum::Generic => std::mem::size_of::<i32>() as u64,
    }
}

// --- Approach B: runtime TypeId ---
#[inline(never)]
pub fn hash_runtime_i32(data: &i32) -> u64 {
    if TypeId::of::<i32>() == TypeId::of::<i32>() {
        let ptr = data as *const i32 as usize;
        (ptr as u64).wrapping_mul(0x517cc1b727220a95)
    } else {
        std::mem::size_of::<i32>() as u64
    }
}

// --- Approach C: dyn dispatch ---
pub trait DynHash {
    fn hash_dyn(&self, data: &i32) -> u64;
}

pub struct DynStaticHasher;
impl DynHash for DynStaticHasher {
    #[inline(never)]
    fn hash_dyn(&self, data: &i32) -> u64 {
        let ptr = data as *const i32 as usize;
        (ptr as u64).wrapping_mul(0x517cc1b727220a95)
    }
}

// --- Approach D: generic fallback ---
#[inline(never)]
pub fn hash_generic_i32(_data: &i32) -> u64 {
    std::mem::size_of::<i32>() as u64
}

// --- Approach E: manual specialization ---
#[inline(never)]
pub fn hash_manual_i32(data: &i32) -> u64 {
    let ptr = data as *const i32 as usize;
    (ptr as u64).wrapping_mul(0x517cc1b727220a95)
}

// --- Bulk functions ---
#[inline(never)]
pub fn sum_hash_enum(items: &[i32]) -> u64 {
    let e = HashEnum::StaticI32;
    let mut total = 0u64;
    for item in items {
        total = total.wrapping_add(hash_enum_i32(&e, item));
    }
    total
}

#[inline(never)]
pub fn sum_hash_runtime(items: &[i32]) -> u64 {
    let mut total = 0u64;
    for item in items {
        total = total.wrapping_add(hash_runtime_i32(item));
    }
    total
}

#[inline(never)]
pub fn sum_hash_dyn(items: &[i32]) -> u64 {
    let d: &dyn DynHash = &DynStaticHasher;
    let mut total = 0u64;
    for item in items {
        total = total.wrapping_add(d.hash_dyn(item));
    }
    total
}

#[inline(never)]
pub fn sum_hash_generic(items: &[i32]) -> u64 {
    let mut total = 0u64;
    for item in items {
        total = total.wrapping_add(hash_generic_i32(item));
    }
    total
}

#[inline(never)]
pub fn sum_hash_manual(items: &[i32]) -> u64 {
    let mut total = 0u64;
    for item in items {
        total = total.wrapping_add(hash_manual_i32(item));
    }
    total
}
