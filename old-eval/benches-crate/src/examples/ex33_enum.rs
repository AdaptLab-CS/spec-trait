// Example 33 — ENUM: Multiple receiver types (3 receivers, same trait)


// --- enum dispatch ---
pub enum CountEnum<'a> {
    VecU8(&'a [u8]),
    SliceU8(&'a [u8]),
    ArrayU8(&'a [u8]),
}

#[inline(never)]
pub fn count_enum(variant: &CountEnum<'_>) -> usize {
    match variant {
        CountEnum::VecU8(data) => {
            let mut c = 0usize;
            for &b in *data {
                c += (b != 0) as usize;
            }
            c
        }
        CountEnum::SliceU8(data) => {
            let mut total = 0usize;
            for &b in *data {
                total = total.wrapping_add(b as usize);
            }
            total
        }
        CountEnum::ArrayU8(data) => {
            let mut acc = 0u8;
            for &b in *data {
                acc ^= b;
            }
            acc as usize
        }
    }
}

