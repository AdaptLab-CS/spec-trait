// Example 08 — RUNTIME TYPEID: Sort
use std::any::TypeId;


// --- runtime TypeId ---
#[inline(never)]
pub fn sort_runtime_u8(data: &mut [u8]) {
    if TypeId::of::<u8>() == TypeId::of::<u8>() {
        let mut counts = [0u32; 256];
        for &b in data.iter() { counts[b as usize] += 1; }
        let mut i = 0;
        for (val, &count) in counts.iter().enumerate() {
            for _ in 0..count { data[i] = val as u8; i += 1; }
        }
    } else {
        data.sort();
    }
}

