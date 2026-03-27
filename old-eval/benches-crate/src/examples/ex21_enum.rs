// Example 21 — ENUM: Conditional combinators (any, all, not)


// --- enum dispatch ---
pub enum ProcessEnum<'a> {
    I32(&'a [i32]),
    I64(&'a [i64]),
    U8(&'a [u8]),
    F64(&'a [f64]),
    VecI32(&'a [Vec<i32>]),
}

#[inline(never)]
pub fn process_enum(variant: &ProcessEnum<'_>) -> usize {
    match variant {
        ProcessEnum::I32(data) => data.len() * 4,
        ProcessEnum::I64(data) => data.len() * 8,
        ProcessEnum::U8(data) => data.len(), // default for u8
        ProcessEnum::F64(data) => data.len() * 8,
        ProcessEnum::VecI32(data) => {
            let mut total = 0;
            for v in *data { total += v.len(); }
            total
        }
    }
}

