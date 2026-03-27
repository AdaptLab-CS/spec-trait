// Example 36 — ENUM: Triple trait bounds (Clone + Eq + Ord)


// --- enum dispatch ---
pub enum SortEnum<'a> {
    I32(&'a [i32]),
    U8(&'a [u8]),
}

#[inline(never)]
pub fn sorted_unique_count_enum(variant: &SortEnum<'_>) -> usize {
    match variant {
        SortEnum::I32(data) => {
            let mut buf: Vec<i32> = data.to_vec();
            buf.sort_unstable();
            buf.dedup();
            buf.len()
        }
        SortEnum::U8(data) => {
            let mut buf: Vec<u8> = data.to_vec();
            buf.sort_unstable();
            buf.dedup();
            buf.len()
        }
    }
}

