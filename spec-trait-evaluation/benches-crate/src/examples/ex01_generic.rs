// Ex01 -- generic fallback (no type knowledge): Histogram
use std::collections::HashMap;

#[inline(never)]
pub fn max_freq_generic_u8(data: &[u8]) -> u32 {
    let mut map = HashMap::<u8, u32>::new();
    for &b in data {
        *map.entry(b).or_insert(0) += 1;
    }
    map.values().copied().max().unwrap_or(0)
}
