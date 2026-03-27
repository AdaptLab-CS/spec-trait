// Ex05 -- enum dispatch: histogram

pub enum HistEnum<'a> {
    U8(&'a [u8]),
}

#[inline(never)]
pub fn histogram_enum(e: &HistEnum<'_>) -> u32 {
    match e {
        HistEnum::U8(data) => {
            let mut counts = [0u32; 256];
            for &b in *data {
                counts[b as usize] += 1;
            }
            counts.iter().copied().max().unwrap_or(0)
        }
    }
}
