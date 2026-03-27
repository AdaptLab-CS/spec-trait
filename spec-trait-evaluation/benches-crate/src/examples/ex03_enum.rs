// Ex03 — enum dispatch: Contains search
pub enum ContainsEnum<'a> {
    U8(&'a [u8], u8),
    I32(&'a [i32], i32),
}

#[inline(never)]
pub fn contains_enum(variant: &ContainsEnum<'_>) -> bool {
    match variant {
        ContainsEnum::U8(data, needle) => memchr::memchr(*needle, data).is_some(),
        ContainsEnum::I32(data, needle) => data.iter().any(|x| x == needle),
    }
}
