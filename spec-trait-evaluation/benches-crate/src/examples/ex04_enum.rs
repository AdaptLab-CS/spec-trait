// Ex04 -- enum dispatch: sort

pub enum SortEnum<'a, T> {
    U8(&'a mut [u8]),
    Generic(&'a mut [T]),
}

pub trait EnumSortTrait<T> {
    fn sort(&mut self);
}

impl<T: Ord> EnumSortTrait<T> for SortEnum<'_, T> {
    #[inline(never)]
    fn sort(&mut self) {
        match self {
            SortEnum::U8(data) => {
                let mut counts = [0u32; 256];
                for &b in data.iter() {
                    counts[b as usize] += 1;
                }
                let mut idx = 0;
                for (val, &cnt) in counts.iter().enumerate() {
                    for _ in 0..cnt {
                        data[idx] = val as u8;
                        idx += 1;
                    }
                }
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
