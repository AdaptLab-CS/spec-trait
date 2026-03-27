// Example 08 — ENUM: Sort


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

