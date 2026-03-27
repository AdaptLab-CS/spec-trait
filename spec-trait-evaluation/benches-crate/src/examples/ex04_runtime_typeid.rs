// Ex04 — runtime TypeId dispatch: Sort
use std::any::TypeId;

#[inline(never)]
pub fn sort_runtime_u8(data: &mut [u8]) {
    if TypeId::of::<u8>() == TypeId::of::<u8>() {
        let mut counts = [0u32; 256];
        for &b in data.iter() {
            counts[b as usize] += 1;
        }
        let mut idx = 0;
        for (val, &cnt) in counts.iter().enumerate() {
            for _ in 0..cnt {
                data[idx] = val as u8;
                idx += 1;
            }
        }
    } else {
        data.sort_unstable();
    }
}
