// Ex07 -- runtime TypeId dispatch: Dedup
use std::any::TypeId;
use std::collections::HashSet;

#[inline(never)]
pub fn dedup_runtime_u8(data: &[u8]) -> Vec<u8> {
    if TypeId::of::<u8>() == TypeId::of::<u8>() {
        let mut seen = [false; 256];
        let mut out = Vec::new();
        for &b in data {
            if !seen[b as usize] {
                seen[b as usize] = true;
                out.push(b);
            }
        }
        out
    } else {
        let mut seen = HashSet::new();
        let mut out = Vec::new();
        for &b in data {
            if seen.insert(b) {
                out.push(b);
            }
        }
        out
    }
}
