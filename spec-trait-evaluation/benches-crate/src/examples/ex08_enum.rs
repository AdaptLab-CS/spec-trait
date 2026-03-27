// Ex08 -- enum dispatch: Count value
pub enum CountValEnum<'a> {
    U8(&'a [u8], u8),
    I32(&'a [i32], i32),
}

#[inline(never)]
pub fn count_val_enum(variant: &CountValEnum<'_>) -> usize {
    match variant {
        CountValEnum::U8(data, needle) => {
            memchr::memchr_iter(*needle, data).count()
        }
        CountValEnum::I32(data, needle) => {
            data.iter().filter(|&&x| x == *needle).count()
        }
    }
}
