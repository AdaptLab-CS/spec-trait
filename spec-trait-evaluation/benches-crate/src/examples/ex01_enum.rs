// Ex01 -- enum dispatch: Histogram
use std::collections::HashMap;

pub enum HistEnum<'a> {
    U8(&'a [u8]),
    I32(&'a [i32]),
}

#[inline(never)]
pub fn max_freq_enum(variant: &HistEnum<'_>) -> u32 {
    match variant {
        HistEnum::U8(data) => {
            let mut counts = [0u32; 256];
            for &b in *data {
                counts[b as usize] += 1;
            }
            counts.iter().copied().max().unwrap_or(0)
        }
        HistEnum::I32(data) => {
            let mut map = HashMap::<i32, u32>::new();
            for &v in *data {
                *map.entry(v).or_insert(0) += 1;
            }
            map.values().copied().max().unwrap_or(0)
        }
    }
}
