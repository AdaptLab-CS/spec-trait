// Example 14 — WITHOUT macros: Min/Max
use std::any::TypeId;

// --- enum ---
pub enum MinMaxEnum<'a> {
    U8(&'a [u8]),
    I32(&'a [i32]),
    F64(&'a [f64]),
}
#[inline(never)]
pub fn minmax_enum_u8(variant: &MinMaxEnum<'_>) -> (u8, u8) {
    match variant {
        MinMaxEnum::U8(data) => {
            let mut min = u8::MAX; let mut max = u8::MIN;
            for &v in *data { if v < min { min = v; } if v > max { max = v; } }
            (min, max)
        }
        _ => (0, 0),
    }
}

#[inline(never)]
pub fn minmax_enum_i32(variant: &MinMaxEnum<'_>) -> (i32, i32) {
    match variant {
        MinMaxEnum::I32(data) => {
            let mut min = i32::MAX; let mut max = i32::MIN;
            for &v in *data { if v < min { min = v; } if v > max { max = v; } }
            (min, max)
        }
        _ => (0, 0),
    }
}

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

// --- generic fallback using Ord ---
#[inline(never)]
pub fn minmax_generic_u8(data: &[u8]) -> (u8, u8) {
    let min = *data.iter().min().unwrap_or(&0);
    let max = *data.iter().max().unwrap_or(&0);
    (min, max)
}

#[inline(never)]
pub fn minmax_generic_i32(data: &[i32]) -> (i32, i32) {
    let min = *data.iter().min().unwrap_or(&0);
    let max = *data.iter().max().unwrap_or(&0);
    (min, max)
}
