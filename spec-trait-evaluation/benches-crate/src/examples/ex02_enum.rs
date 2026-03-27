// Ex02 -- enum dispatch: Set intersection
use std::collections::HashSet;

pub enum IntersectEnum<'a> {
    U8(&'a [u8], &'a [u8]),
    I32(&'a [i32], &'a [i32]),
}

#[inline(never)]
pub fn intersect_enum(variant: &IntersectEnum<'_>) -> usize {
    match variant {
        IntersectEnum::U8(a, b) => {
            let mut present = [false; 256];
            for &v in *a {
                present[v as usize] = true;
            }
            b.iter().filter(|&&v| present[v as usize]).count()
        }
        IntersectEnum::I32(a, b) => {
            let set: HashSet<i32> = a.iter().copied().collect();
            b.iter().filter(|x| set.contains(x)).count()
        }
    }
}
