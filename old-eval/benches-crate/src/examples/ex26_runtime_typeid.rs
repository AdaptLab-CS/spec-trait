// Example 26 — RUNTIME TYPEID: Combined trait bounds (T: Copy + Eq)
use std::any::TypeId;


// --- Approach B: runtime TypeId ---
#[inline(never)]
pub fn dedup_runtime_u8(data: &[u8]) -> usize {
    if TypeId::of::<u8>() == TypeId::of::<u8>() {
        let mut seen = [false; 256];
        let mut count = 0usize;
        for &b in data {
            if !seen[b as usize] {
                seen[b as usize] = true;
                count += 1;
            }
        }
        count
    } else {
        data.len()
    }
}

#[inline(never)]
pub fn dedup_runtime_i32(data: &[i32]) -> usize {
    if TypeId::of::<i32>() == TypeId::of::<i32>() {
        let mut buf: Vec<i32> = Vec::with_capacity(data.len());
        let mut count = 0usize;
        for &item in data {
            if !buf.contains(&item) {
                buf.push(item);
                count += 1;
            }
        }
        count
    } else {
        data.len()
    }
}

