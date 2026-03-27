// Ex01 -- runtime TypeId dispatch: Histogram
use std::any::TypeId;
use std::collections::HashMap;

#[inline(never)]
pub fn max_freq_runtime_u8(data: &[u8]) -> u32 {
    if TypeId::of::<u8>() == TypeId::of::<u8>() {
        let mut counts = [0u32; 256];
        for &b in data {
            counts[b as usize] += 1;
        }
        counts.iter().copied().max().unwrap_or(0)
    } else {
        let mut map = HashMap::<u8, u32>::new();
        for &b in data {
            *map.entry(b).or_insert(0) += 1;
        }
        map.values().copied().max().unwrap_or(0)
    }
}
