// Ex08 -- enum dispatch: count_value

pub enum CountValEnum<'a> {
    U8(&'a [u8], u8),
}

#[inline(never)]
pub fn count_val_enum(e: &CountValEnum<'_>) -> usize {
    match e {
        CountValEnum::U8(data, needle) => {
            memchr::memchr_iter(*needle, data).count()
        }
    }
}
