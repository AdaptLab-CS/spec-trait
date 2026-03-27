// Example 18 — ENUM: HRTB — higher-order function application


// --- enum dispatch ---
pub enum ApplyEnum<'a> {
    I32(fn(&i32) -> i32, &'a [i32]),
    F64(fn(&f64) -> f64, &'a [f64]),
}

#[inline(never)]
pub fn apply_enum_i32(variant: &ApplyEnum<'_>) -> Vec<i32> {
    match variant {
        ApplyEnum::I32(f, data) => data.iter().map(|x| f(x)).collect(),
        _ => unreachable!(),
    }
}

#[inline(never)]
pub fn apply_enum_f64(variant: &ApplyEnum<'_>) -> Vec<f64> {
    match variant {
        ApplyEnum::F64(f, data) => data.iter().map(|x| f(x)).collect(),
        _ => unreachable!(),
    }
}

