// Ex03 -- enum dispatch: contains

pub enum ContainsEnum<'a, T> {
    U8(&'a [u8], u8),
    Generic(&'a [T], T),
}

pub trait EnumContainsTrait<T> {
    fn contains(&self) -> bool;
}

impl<T: PartialEq> EnumContainsTrait<T> for ContainsEnum<'_, T> {
    #[inline(never)]
    fn contains(&self) -> bool {
        match self {
            ContainsEnum::U8(data, needle) => {
                memchr::memchr(*needle, data).is_some()
            }
            ContainsEnum::Generic(data, needle) => {
                data.iter().any(|x| x == needle)
            }
        }
    }
}

pub fn contains_enum(e: &ContainsEnum<u8>) -> bool {
    e.contains()
}
