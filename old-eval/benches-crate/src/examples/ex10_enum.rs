// Example 10 — ENUM: Min/Max


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

