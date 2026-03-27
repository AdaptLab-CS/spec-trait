// Ex06 -- enum dispatch: dedup

pub enum DedupEnum<'a> {
    U8(&'a [u8]),
}

#[inline(never)]
pub fn dedup_enum(e: &DedupEnum<'_>) -> Vec<u8> {
    match e {
        DedupEnum::U8(data) => {
            let mut seen = [false; 256];
            let mut out = Vec::new();
            for &b in *data {
                if !seen[b as usize] {
                    seen[b as usize] = true;
                    out.push(b);
                }
            }
            out
        }
    }
}
