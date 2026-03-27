// Ex05 -- runtime TypeId dispatch: histogram
use std::any::TypeId;
use std::collections::HashMap;
use std::hash::Hash;

#[inline(never)]
pub fn max_freq_runtime<T: Eq + Hash + 'static>(data: &[T]) -> u32 {
    if TypeId::of::<T>() == TypeId::of::<u8>() {
        let slice = unsafe {
            std::slice::from_raw_parts(data.as_ptr() as *const u8, data.len())
        };
        let mut counts = [0u32; 256];
        for &b in slice {
            counts[b as usize] += 1;
        }
        counts.iter().copied().max().unwrap_or(0)
    } else {
        let mut map = HashMap::<&T, u32>::new();
        for item in data {
            *map.entry(item).or_insert(0) += 1;
        }
        map.values().copied().max().unwrap_or(0)
    }
}
