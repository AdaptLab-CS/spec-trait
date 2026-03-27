// Example 11 — ENUM: Contains / search


// --- enum ---
pub enum ContainsEnum<'a> {
    U8(&'a [u8], u8),
    I32(&'a [i32], i32),
}
#[inline(never)]
pub fn contains_enum(variant: &ContainsEnum<'_>) -> bool {
    match variant {
        ContainsEnum::U8(haystack, needle) => memchr::memchr(*needle, haystack).is_some(),
        ContainsEnum::I32(haystack, needle) => haystack.iter().any(|x| x == needle),
    }
}

