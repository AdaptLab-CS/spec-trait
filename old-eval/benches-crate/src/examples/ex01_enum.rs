// Example 01 — ENUM: Sum


// --- enum dispatch ---
pub enum SumEnum<'a> {
    I32(&'a [i32]),
    F64(&'a [f64]),
    U8(&'a [u8]),
}

#[inline(never)]
pub fn sum_enum(variant: &SumEnum<'_>) -> f64 {
    match variant {
        SumEnum::I32(d) => d.iter().copied().sum::<i32>() as f64,
        SumEnum::F64(d) => d.iter().copied().sum(),
        SumEnum::U8(d) => {
            let mut acc: u8 = 0;
            for &b in *d { acc = acc.wrapping_add(b); }
            acc as f64
        }
    }
}

