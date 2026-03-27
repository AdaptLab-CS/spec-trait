// Ex07 -- enum dispatch: Dedup
use std::collections::HashSet;

pub enum DedupEnum<'a> {
    U8(&'a [u8]),
    I32(&'a [i32]),
}

#[inline(never)]
pub fn dedup_enum(variant: &DedupEnum<'_>) -> usize {
    match variant {
        DedupEnum::U8(data) => {
            let mut seen = [false; 256];
            let mut count = 0usize;
            for &b in *data {
                if !seen[b as usize] {
                    seen[b as usize] = true;
                    count += 1;
                }
            }
            count
        }
        DedupEnum::I32(data) => {
            let mut seen = HashSet::new();
            let mut count = 0usize;
            for &v in *data {
                if seen.insert(v) {
                    count += 1;
                }
            }
            count
        }
    }
}
