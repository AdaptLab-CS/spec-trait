// Example 29 — RUNTIME TYPEID: Nested generic type specialization
use std::any::TypeId;


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

