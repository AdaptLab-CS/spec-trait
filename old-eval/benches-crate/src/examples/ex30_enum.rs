// Example 30 — ENUM: any() predicate – 4-way integer fast path


// --- enum dispatch ---
pub enum FilterEnum<'a> {
    I32(&'a [i32]),
    U64(&'a [u64]),
    U8(&'a [u8]),
    F64(&'a [f64]),
}

#[inline(never)]
pub fn filter_count_enum(variant: &FilterEnum<'_>) -> usize {
    match variant {
        FilterEnum::I32(data) => data.len() * std::mem::size_of::<i32>(),
        FilterEnum::U64(data) => data.len() * std::mem::size_of::<u64>(),
        FilterEnum::U8(data) => {
            let mut count = 0usize;
            for &b in *data {
                count += (b != 0) as usize;
            }
            count
        }
        FilterEnum::F64(data) => data.len() * 8,
    }
}

