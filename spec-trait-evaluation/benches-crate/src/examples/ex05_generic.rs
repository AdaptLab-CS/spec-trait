// Ex05 -- generic fallback (no type knowledge): Distinct count
use std::collections::HashSet;

#[inline(never)]
pub fn distinct_generic_u8(data: &[u8]) -> usize {
    let set: HashSet<u8> = data.iter().copied().collect();
    set.len()
}
