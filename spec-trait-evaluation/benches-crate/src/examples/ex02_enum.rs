// Ex02 -- enum dispatch: to_bytes

pub enum ToBytesEnum<'a> {
    U8(&'a [u8]),
}

#[inline(never)]
pub fn to_bytes_enum(e: &ToBytesEnum<'_>) -> Vec<u8> {
    match e {
        ToBytesEnum::U8(data) => {
            data.to_vec()
        }
    }
}
