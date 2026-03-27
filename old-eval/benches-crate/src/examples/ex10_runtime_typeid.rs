// Example 10 — RUNTIME TYPEID: Min/Max
use std::any::TypeId;


// --- runtime TypeId ---
#[inline(never)]
pub fn minmax_runtime_u8(data: &[u8]) -> (u8, u8) {
    if TypeId::of::<u8>() == TypeId::of::<u8>() {
        let mut min = u8::MAX; let mut max = u8::MIN;
        for &v in data { if v < min { min = v; } if v > max { max = v; } }
        (min, max)
    } else { (0, 0) }
}

#[inline(never)]
pub fn minmax_runtime_i32(data: &[i32]) -> (i32, i32) {
    if TypeId::of::<i32>() == TypeId::of::<i32>() {
        let mut min = i32::MAX; let mut max = i32::MIN;
        for &v in data { if v < min { min = v; } if v > max { max = v; } }
        (min, max)
    } else { (0, 0) }
}

