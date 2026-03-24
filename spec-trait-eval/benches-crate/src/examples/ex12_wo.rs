// Example 12 — WITHOUT macros: Sort
use std::any::TypeId;

// --- enum dispatch ---
pub enum SortEnum<'a> {
    U8(&'a mut [u8]),
    I32(&'a mut [i32]),
}
#[inline(never)]
pub fn sort_enum(variant: &mut SortEnum<'_>) {
    match variant {
        SortEnum::U8(data) => {
            let mut counts = [0u32; 256];
            for &b in data.iter() { counts[b as usize] += 1; }
            let mut i = 0;
            for (val, &count) in counts.iter().enumerate() {
                for _ in 0..count { data[i] = val as u8; i += 1; }
            }
        }
        SortEnum::I32(data) => data.sort_unstable(),
    }
}

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

// --- generic fallback ---
#[inline(never)]
pub fn sort_generic_u8(data: &mut [u8]) {
    data.sort();
}

#[inline(never)]
pub fn sort_generic_i32(data: &mut [i32]) {
    data.sort();
}

// --- counting sort (manual specialized) ---
#[inline(never)]
pub fn sort_counting_u8(data: &mut [u8]) {
    let mut counts = [0u32; 256];
    for &b in data.iter() { counts[b as usize] += 1; }
    let mut i = 0;
    for (val, &count) in counts.iter().enumerate() {
        for _ in 0..count { data[i] = val as u8; i += 1; }
    }
}
