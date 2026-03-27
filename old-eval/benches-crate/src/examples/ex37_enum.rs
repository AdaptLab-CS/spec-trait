// Example 37 — ENUM: Generic receiver + type specialization


// --- enum dispatch ---
pub enum AccumEnum<'a> {
    I32(&'a [i32]),
    U8(&'a [u8]),
}

#[inline(never)]
pub fn accum_count_enum(variant: &AccumEnum<'_>) -> usize {
    match variant {
        AccumEnum::I32(data) => {
            let mut acc = 0usize;
            for &x in *data {
                acc = acc.wrapping_add((x as i64).unsigned_abs() as usize);
            }
            acc
        }
        AccumEnum::U8(data) => {
            let mut acc = 0u64;
            for &b in *data {
                acc = acc.saturating_add(b as u64);
            }
            acc as usize
        }
    }
}

