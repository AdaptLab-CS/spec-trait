// Ex08 -- enum dispatch: count_value

pub enum CountValEnum<'a, T> {
    U8(&'a [u8], u8),
    Generic(&'a [T], T),
}

#[inline(never)]
fn matches_u8_direct(item: &u8, needle: &u8) -> bool {
    *item == *needle
}

pub trait EnumCountValTrait<T> {
    fn count_val(&self) -> usize;
}

impl<T: PartialEq> EnumCountValTrait<T> for CountValEnum<'_, T> {
    #[inline(never)]
    fn count_val(&self) -> usize {
        match self {
            CountValEnum::U8(data, needle) => {
                let mut count = 0;
                for item in *data {
                    if matches_u8_direct(item, needle) {
                        count += 1;
                    }
                }
                count
            }
            CountValEnum::Generic(data, needle) => {
                data.iter().filter(|x| *x == needle).count()
            }
        }
    }
}

pub fn count_val_enum(e: &CountValEnum<u8>) -> usize {
    e.count_val()
}
