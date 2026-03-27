// Example 20 — ENUM: Wildcard and complex type patterns


// --- enum dispatch ---
pub enum MeasureEnum<'a> {
    VecI32(&'a Vec<i32>),
    VecU8(&'a Vec<u8>),
    Tuple(&'a (i32, f64)),
    Slice(&'a [i32]),
}

#[inline(never)]
pub fn measure_enum(variant: &MeasureEnum<'_>) -> usize {
    match variant {
        MeasureEnum::VecI32(_) => std::mem::size_of::<Vec<i32>>() + 24,
        MeasureEnum::VecU8(_) => std::mem::size_of::<Vec<u8>>() + 24,
        MeasureEnum::Tuple(_) => 4 + std::mem::size_of::<(i32, f64)>() - 4,
        MeasureEnum::Slice(_) => 16,
    }
}

