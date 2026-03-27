// Ex05 -- enum dispatch: Distinct count
use std::collections::HashSet;

pub enum DistinctEnum<'a> {
    U8(&'a [u8]),
    I32(&'a [i32]),
}

#[inline(never)]
pub fn distinct_enum(variant: &DistinctEnum<'_>) -> usize {
    match variant {
        DistinctEnum::U8(data) => {
            let mut seen = [false; 256];
            for &b in *data {
                seen[b as usize] = true;
            }
            seen.iter().filter(|&&v| v).count()
        }
        DistinctEnum::I32(data) => {
            let set: HashSet<i32> = data.iter().copied().collect();
            set.len()
        }
    }
}
