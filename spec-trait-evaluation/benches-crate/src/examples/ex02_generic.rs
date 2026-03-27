// Ex02 -- generic fallback (no type knowledge): Set intersection
use std::collections::HashSet;

#[inline(never)]
pub fn intersect_generic_u8(a: &[u8], b: &[u8]) -> usize {
    let set: HashSet<u8> = a.iter().copied().collect();
    b.iter().filter(|x| set.contains(x)).count()
}
