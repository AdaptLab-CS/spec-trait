// Example 14 — ENUM: FMA


// --- enum ---
pub enum FmaEnum<'a> {
    F64(&'a [f64], f64, f64),
    F32(&'a [f32], f32, f32),
    I32(&'a [i32], i32, i32),
}
#[inline(never)]
pub fn fma_enum_f64(variant: &FmaEnum<'_>) -> Vec<f64> {
    match variant {
        FmaEnum::F64(data, mul, add) => data.iter().map(|x| x.mul_add(*mul, *add)).collect(),
        _ => vec![],
    }
}

#[inline(never)]
pub fn fma_enum_i32(variant: &FmaEnum<'_>) -> Vec<i32> {
    match variant {
        FmaEnum::I32(data, mul, add) => data.iter().map(|x| x.wrapping_mul(*mul).wrapping_add(*add)).collect(),
        _ => vec![],
    }
}

