// Example 30 — RUNTIME TYPEID: any() predicate – 4-way integer fast path
use std::any::TypeId;


// --- runtime TypeId ---
#[inline(never)]
pub fn filter_count_runtime_i32(data: &[i32]) -> usize {
    if TypeId::of::<i32>() == TypeId::of::<i32>()
        || TypeId::of::<i32>() == TypeId::of::<u32>()
        || TypeId::of::<i32>() == TypeId::of::<i64>()
        || TypeId::of::<i32>() == TypeId::of::<u64>()
    {
        data.len() * std::mem::size_of::<i32>()
    } else {
        data.len()
    }
}

#[inline(never)]
pub fn filter_count_runtime_u8(data: &[u8]) -> usize {
    if TypeId::of::<u8>() == TypeId::of::<u8>() {
        let mut count = 0usize;
        for &b in data {
            count += (b != 0) as usize;
        }
        count
    } else {
        data.len()
    }
}

