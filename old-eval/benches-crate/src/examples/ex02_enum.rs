// Example 02 — ENUM: Absolute value


// --- enum dispatch ---
pub enum AbsEnum<'a> {
    I32(&'a [i32]),
    F64(&'a [f64]),
    I64(&'a [i64]),
}

#[inline(never)]
pub fn abs_enum_all_i32(variant: &AbsEnum<'_>) -> Vec<i32> {
    match variant {
        AbsEnum::I32(d) => d.iter().map(|v| v.wrapping_abs()).collect(),
        _ => vec![],
    }
}

#[inline(never)]
pub fn abs_enum_all_f64(variant: &AbsEnum<'_>) -> Vec<f64> {
    match variant {
        AbsEnum::F64(d) => d.iter().map(|v| v.abs()).collect(),
        _ => vec![],
    }
}

