// Example 36 — RUNTIME TYPEID: Triple trait bounds (Clone + Eq + Ord)
use std::any::TypeId;


// --- runtime TypeId ---
#[inline(never)]
pub fn sorted_unique_count_runtime_i32(data: &[i32]) -> usize {
    if TypeId::of::<i32>() == TypeId::of::<i32>() {
        let mut buf: Vec<i32> = data.to_vec();
        buf.sort_unstable();
        buf.dedup();
        buf.len()
    } else {
        data.len()
    }
}

#[inline(never)]
pub fn sorted_unique_count_runtime_u8(data: &[u8]) -> usize {
    if TypeId::of::<u8>() == TypeId::of::<u8>() {
        let mut buf: Vec<u8> = data.to_vec();
        buf.sort_unstable();
        buf.dedup();
        buf.len()
    } else {
        data.len()
    }
}

