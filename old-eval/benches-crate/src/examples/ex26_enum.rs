// Example 26 — ENUM: Combined trait bounds (T: Copy + Eq)


// --- Approach A: enum dispatch ---
pub enum DedupEnum<'a> {
    U8(&'a [u8]),
    I32(&'a [i32]),
}

#[inline(never)]
pub fn dedup_enum(variant: &DedupEnum) -> usize {
    match variant {
        DedupEnum::U8(data) => {
            let mut seen = [false; 256];
            let mut count = 0usize;
            for &b in *data {
                let idx = b as usize;
                if !seen[idx] {
                    seen[idx] = true;
                    count += 1;
                }
            }
            count
        }
        DedupEnum::I32(data) => {
            let mut buf: Vec<i32> = Vec::with_capacity(data.len());
            let mut count = 0usize;
            for &item in *data {
                if !buf.contains(&item) {
                    buf.push(item);
                    count += 1;
                }
            }
            count
        }
    }
}

