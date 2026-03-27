// Ex02 -- enum dispatch: to_bytes
use std::fmt::Display;

pub enum ToBytesEnum<'a, T> {
    U8(&'a [u8]),
    Generic(&'a [T]),
}

pub trait EnumToByteTrait<T> {
    fn to_bytes(&self) -> Vec<u8>;
}

impl<T: Display> EnumToByteTrait<T> for ToBytesEnum<'_, T> {
    #[inline(never)]
    fn to_bytes(&self) -> Vec<u8> {
        match self {
            ToBytesEnum::U8(data) => data.to_vec(),
            ToBytesEnum::Generic(data) => {
                let mut result = Vec::new();
                for item in *data {
                    result.extend_from_slice(format!("{}", item).as_bytes());
                }
                result
            }
        }
    }
}

pub fn to_bytes_enum(e: &ToBytesEnum<u8>) -> Vec<u8> {
    e.to_bytes()
}
