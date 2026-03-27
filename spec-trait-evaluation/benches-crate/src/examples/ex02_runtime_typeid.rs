// Ex02 -- runtime TypeId dispatch: Set intersection
use std::any::TypeId;
use std::collections::HashSet;

#[inline(never)]
pub fn intersect_runtime_u8(a: &[u8], b: &[u8]) -> usize {
    if TypeId::of::<u8>() == TypeId::of::<u8>() {
        let mut present = [false; 256];
        for &v in a {
            present[v as usize] = true;
        }
        b.iter().filter(|&&v| present[v as usize]).count()
    } else {
        let set: HashSet<u8> = a.iter().copied().collect();
        b.iter().filter(|x| set.contains(x)).count()
    }
}
