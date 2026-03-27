// Example 34 — RUNTIME TYPEID: 'static + Copy compound bound
use std::any::TypeId;


// --- runtime TypeId ---
#[inline(never)]
pub fn cached_size_runtime_i32(data: &[i32]) -> usize {
    if TypeId::of::<i32>() == TypeId::of::<i32>() {
        let mut acc = 0usize;
        for &x in data {
            acc = acc.wrapping_add((x as i64).unsigned_abs() as usize);
        }
        acc
    } else {
        data.len()
    }
}

#[inline(never)]
pub fn cached_size_runtime_u8(data: &[u8]) -> usize {
    if TypeId::of::<u8>() == TypeId::of::<u8>() {
        data.len() + 8
    } else {
        data.len()
    }
}

