// Example 33 — RUNTIME TYPEID: Multiple receiver types (3 receivers, same trait)
use std::any::TypeId;


// --- runtime TypeId ---
#[inline(never)]
pub fn count_runtime_vec_u8(data: &[u8]) -> usize {
    if TypeId::of::<u8>() == TypeId::of::<u8>() {
        let mut c = 0usize;
        for &b in data {
            c += (b != 0) as usize;
        }
        c
    } else {
        data.len()
    }
}

#[inline(never)]
pub fn count_runtime_slice_u8(data: &[u8]) -> usize {
    if TypeId::of::<u8>() == TypeId::of::<u8>() {
        let mut total = 0usize;
        for &b in data {
            total = total.wrapping_add(b as usize);
        }
        total
    } else {
        data.len() * 2
    }
}

