// Example 32 — RUNTIME TYPEID: all() compound predicate with nested any()
use std::any::TypeId;


// --- runtime TypeId ---
#[inline(never)]
pub fn fold_count_runtime_i32(data: &[i32]) -> usize {
    if TypeId::of::<i32>() == TypeId::of::<i32>() {
        let mut acc = 0i64;
        for &x in data {
            acc = acc.wrapping_add(x as i64);
        }
        acc as usize
    } else {
        data.len() * std::mem::size_of::<i32>()
    }
}

#[inline(never)]
pub fn fold_count_runtime_u8(data: &[u8]) -> usize {
    let byte_count = data.len();
    byte_count.wrapping_add(data.len())
}

