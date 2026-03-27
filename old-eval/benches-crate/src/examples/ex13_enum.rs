// Example 13 — ENUM: Clamping


// --- enum ---
pub enum ClampEnum<'a> {
    I32(&'a [i32], i32, i32),
    F64(&'a [f64], f64, f64),
    U8(&'a [u8], u8, u8),
}
#[inline(never)]
pub fn clamp_enum_i32(variant: &ClampEnum<'_>) -> Vec<i32> {
    match variant {
        ClampEnum::I32(data, lo, hi) => data.iter().map(|&x| x.clamp(*lo, *hi)).collect(),
        _ => vec![],
    }
}

#[inline(never)]
pub fn clamp_enum_f64(variant: &ClampEnum<'_>) -> Vec<f64> {
    match variant {
        ClampEnum::F64(data, lo, hi) => data.iter().map(|&x| x.clamp(*lo, *hi)).collect(),
        _ => vec![],
    }
}

