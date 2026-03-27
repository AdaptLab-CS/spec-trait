// Example 32 — ENUM: all() compound predicate with nested any()


// --- enum dispatch ---
pub enum FoldEnum<'a> {
    I32(&'a [i32]),
    U8(&'a [u8]),
    F64(&'a [f64]),
}

#[inline(never)]
pub fn fold_count_enum(variant: &FoldEnum<'_>) -> usize {
    match variant {
        FoldEnum::I32(data) => {
            let mut acc = 0i64;
            for &x in *data {
                acc = acc.wrapping_add(x as i64);
            }
            acc as usize
        }
        FoldEnum::U8(data) => {
            let byte_count = data.len();
            byte_count.wrapping_add(data.len())
        }
        FoldEnum::F64(data) => data.len() * 8,
    }
}

