// Ex05 -- naive: histogram — HashMap (no specialization)
use std::collections::HashMap;
use std::hash::Hash;

#[inline(never)]
pub fn max_freq_naive<T: Eq + Hash>(data: &[T]) -> u32 {
    let mut map = HashMap::<&T, u32>::new();
    for item in data {
        *map.entry(item).or_insert(0) += 1;
    }
    map.values().copied().max().unwrap_or(0)
}
