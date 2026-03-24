// Example 33 — WITHOUT macros: Nested generic type specialization
use std::any::TypeId;

// --- Approach A: enum dispatch ---
pub enum FlatEnum<'a> {
    VecVecU8(&'a [Vec<Vec<u8>>]),
    VecString(&'a [Vec<String>]),
    VecI32(&'a [Vec<i32>]),
}

#[inline(never)]
pub fn flat_enum(variant: &FlatEnum) -> usize {
    match variant {
        FlatEnum::VecVecU8(data) => {
            let mut total = 0usize;
            for outer in *data {
                for inner in outer {
                    total += inner.len();
                }
            }
            total
        }
        FlatEnum::VecString(data) => {
            let mut total = 0usize;
            for outer in *data {
                for s in outer {
                    total += s.len();
                }
            }
            total
        }
        FlatEnum::VecI32(data) => {
            let mut total = 0usize;
            for outer in *data {
                total += outer.len() * 4;
            }
            total
        }
    }
}

// --- Approach B: runtime TypeId ---
#[inline(never)]
pub fn flat_runtime_vec_vec_u8(data: &[Vec<Vec<u8>>]) -> usize {
    if TypeId::of::<Vec<Vec<u8>>>() == TypeId::of::<Vec<Vec<u8>>>() {
        let mut total = 0usize;
        for outer in data {
            for inner in outer {
                total += inner.len();
            }
        }
        total
    } else {
        data.len()
    }
}

#[inline(never)]
pub fn flat_runtime_vec_i32(data: &[Vec<i32>]) -> usize {
    if TypeId::of::<Vec<i32>>() == TypeId::of::<Vec<i32>>() {
        let mut total = 0usize;
        for outer in data {
            total += outer.len() * 4;
        }
        total
    } else {
        data.len()
    }
}

// --- Approach C: generic fallback ---
#[inline(never)]
pub fn flat_generic_vec_vec_u8(data: &[Vec<Vec<u8>>]) -> usize {
    data.len()
}

#[inline(never)]
pub fn flat_generic_vec_i32(data: &[Vec<i32>]) -> usize {
    data.len()
}

// --- Approach D: manual specialization ---
#[inline(never)]
pub fn flat_manual_vec_vec_u8(data: &[Vec<Vec<u8>>]) -> usize {
    let mut total = 0usize;
    for outer in data {
        for inner in outer {
            total += inner.len();
        }
    }
    total
}

#[inline(never)]
pub fn flat_manual_vec_i32(data: &[Vec<i32>]) -> usize {
    let mut total = 0usize;
    for outer in data {
        total += outer.len() * 4;
    }
    total
}

// --- Approach E: dyn dispatch ---
pub trait DynFlat {
    fn flat_dyn(&self) -> usize;
}

pub struct DynVecVecU8<'a>(pub &'a [Vec<Vec<u8>>]);
impl DynFlat for DynVecVecU8<'_> {
    #[inline(never)]
    fn flat_dyn(&self) -> usize {
        let mut total = 0usize;
        for outer in self.0 {
            for inner in outer {
                total += inner.len();
            }
        }
        total
    }
}

pub struct DynVecI32<'a>(pub &'a [Vec<i32>]);
impl DynFlat for DynVecI32<'_> {
    #[inline(never)]
    fn flat_dyn(&self) -> usize {
        let mut total = 0usize;
        for outer in self.0 {
            total += outer.len() * 4;
        }
        total
    }
}
