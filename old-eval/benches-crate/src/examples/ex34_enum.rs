// Example 34 — ENUM: 'static + Copy compound bound


// --- enum dispatch ---
pub enum CacheEnum<'a> {
    I32(&'a [i32]),
    U8(&'a [u8]),
    Strings(&'a [String]),
}

#[inline(never)]
pub fn cached_size_enum(variant: &CacheEnum<'_>) -> usize {
    match variant {
        CacheEnum::I32(data) => {
            let mut acc = 0usize;
            for &x in *data {
                acc = acc.wrapping_add((x as i64).unsigned_abs() as usize);
            }
            acc
        }
        CacheEnum::U8(data) => data.len() + 8,
        CacheEnum::Strings(data) => data.len() * std::mem::size_of::<String>(),
    }
}

