// Ex07 -- generic fallback: Dedup (full dedup via HashSet)
use std::collections::HashSet;

#[inline(never)]
pub fn dedup_generic_u8(data: &[u8]) -> Vec<u8> {
    let mut seen = HashSet::new();
    let mut out = Vec::new();
    for &b in data {
        if seen.insert(b) {
            out.push(b);
        }
    }
    out
}
