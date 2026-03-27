// Example 27 — RUNTIME TYPEID: Many specializations (8+ types)
use std::any::TypeId;


// --- Approach B: runtime TypeId ---
#[inline(never)]
pub fn encode_runtime_u8(data: &[u8]) -> usize {
    if TypeId::of::<u8>() == TypeId::of::<u8>() {
        data.len()
    } else {
        data.len() * std::mem::size_of::<u8>()
    }
}

#[inline(never)]
pub fn encode_runtime_i32(data: &[i32]) -> usize {
    if TypeId::of::<i32>() == TypeId::of::<i32>() {
        let mut total = 0usize;
        for &v in data {
            let uv = v as u32 as usize;
            total += if uv < 128 { 1 } else if uv < 16384 { 2 } else { 4 };
        }
        total
    } else {
        data.len() * 4
    }
}

