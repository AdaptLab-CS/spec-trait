// Ex05 -- runtime TypeId dispatch: Distinct count
use std::any::TypeId;
use std::collections::HashSet;

#[inline(never)]
pub fn distinct_runtime_u8(data: &[u8]) -> usize {
    if TypeId::of::<u8>() == TypeId::of::<u8>() {
        let mut seen = [false; 256];
        for &b in data {
            seen[b as usize] = true;
        }
        seen.iter().filter(|&&v| v).count()
    } else {
        let set: HashSet<u8> = data.iter().copied().collect();
        set.len()
    }
}
