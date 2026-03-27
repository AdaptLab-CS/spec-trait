// Ex03 -- enum dispatch: contains

pub enum ContainsEnum<'a> {
    U8(&'a [u8], u8),
}

#[inline(never)]
pub fn contains_enum(e: &ContainsEnum<'_>) -> bool {
    match e {
        ContainsEnum::U8(data, needle) => {
            memchr::memchr(*needle, data).is_some()
        }
    }
}
