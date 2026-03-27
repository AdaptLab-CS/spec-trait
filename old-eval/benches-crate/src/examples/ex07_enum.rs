// Example 07 — ENUM: Map / transform


// --- enum dispatch ---
pub enum MapEnum<'a> {
    I32(&'a [i32]),
    F64(&'a [f64]),
    U8(&'a [u8]),
}
#[inline(never)]
pub fn map_enum_i32(variant: &MapEnum<'_>) -> Vec<i32> {
    match variant {
        MapEnum::I32(d) => d.iter().map(|x| x.wrapping_mul(2)).collect(),
        _ => vec![],
    }
}

#[inline(never)]
pub fn map_enum_f64(variant: &MapEnum<'_>) -> Vec<f64> {
    match variant {
        MapEnum::F64(d) => d.iter().map(|x| x * 2.0).collect(),
        _ => vec![],
    }
}

#[inline(never)]
pub fn map_enum_u8(variant: &MapEnum<'_>) -> Vec<u8> {
    match variant {
        MapEnum::U8(d) => d.iter().map(|x| x.saturating_mul(2)).collect(),
        _ => vec![],
    }
}

