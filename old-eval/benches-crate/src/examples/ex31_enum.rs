// Example 31 — ENUM: not() predicate – type exclusion


// --- enum dispatch ---
pub enum NormEnum<'a> {
    I32(&'a [i32]),
    U8(&'a [u8]),
    Strings(&'a [String]),
}

#[inline(never)]
pub fn compute_enum(variant: &NormEnum<'_>) -> usize {
    match variant {
        NormEnum::I32(data) => data.len() * std::mem::size_of::<i32>(),
        NormEnum::U8(data) => data.len(),
        NormEnum::Strings(data) => {
            let mut total = 0usize;
            for s in *data {
                total += s.len();
            }
            total
        }
    }
}

