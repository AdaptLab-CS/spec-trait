// Example 16 — ENUM: Dot product


// --- enum ---
pub enum DotEnum<'a> {
    F64(&'a [f64], &'a [f64]),
    F32(&'a [f32], &'a [f32]),
    I32(&'a [i32], &'a [i32]),
}
#[inline(never)]
pub fn dot_enum_f64(variant: &DotEnum<'_>) -> f64 {
    match variant {
        DotEnum::F64(a, b) => a.iter().zip(b.iter()).map(|(x, y)| x * y).sum(),
        _ => 0.0,
    }
}

#[inline(never)]
pub fn dot_enum_f32(variant: &DotEnum<'_>) -> f32 {
    match variant {
        DotEnum::F32(a, b) => a.iter().zip(b.iter()).map(|(x, y)| x * y).sum(),
        _ => 0.0,
    }
}

#[inline(never)]
pub fn dot_enum_i32(variant: &DotEnum<'_>) -> i32 {
    match variant {
        DotEnum::I32(a, b) => a.iter().zip(b.iter()).map(|(x, y)| x.wrapping_mul(*y)).sum(),
        _ => 0,
    }
}

