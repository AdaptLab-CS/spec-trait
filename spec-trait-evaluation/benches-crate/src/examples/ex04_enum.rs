// Ex04 -- enum dispatch: sort

pub enum SortEnum<'a> {
    U8(&'a mut [u8]),
}

#[inline(never)]
pub fn sort_enum(e: &mut SortEnum<'_>) {
    match e {
        SortEnum::U8(data) => {
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
        }
    }
}
