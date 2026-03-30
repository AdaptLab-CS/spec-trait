// Ex04 -- enum dispatch: sort
use std::cmp::Ordering;

pub enum SortEnum<'a, T> {
    U8(&'a mut [u8]),
    Generic(&'a mut [T]),
}

#[inline(never)]
fn compare_u8_direct(a: &u8, b: &u8) -> Ordering {
    if *a < *b {
        Ordering::Less
    } else if *a > *b {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

pub trait EnumSortTrait<T> {
    fn sort(&mut self);
}

impl<T: Ord> EnumSortTrait<T> for SortEnum<'_, T> {
    #[inline(never)]
    fn sort(&mut self) {
        match self {
            SortEnum::U8(data) => {
                data.sort_unstable_by(|a, b| compare_u8_direct(a, b));
            }
            SortEnum::Generic(data) => {
                data.sort_unstable();
            }
        }
    }
}

pub fn sort_enum(e: &mut SortEnum<u8>) {
    e.sort();
}
