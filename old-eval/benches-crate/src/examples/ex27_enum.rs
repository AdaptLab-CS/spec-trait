// Example 27 — ENUM: Many specializations (8+ types)


// --- Approach A: enum dispatch ---
pub enum EncodeEnum<'a> {
    U8(&'a [u8]),
    I32(&'a [i32]),
    F64(&'a [f64]),
    U64(&'a [u64]),
}

#[inline(never)]
pub fn encode_enum(variant: &EncodeEnum) -> usize {
    match variant {
        EncodeEnum::U8(data) => data.len(),
        EncodeEnum::I32(data) => {
            let mut total = 0usize;
            for &v in *data {
                let uv = v as u32 as usize;
                total += if uv < 128 { 1 } else if uv < 16384 { 2 } else { 4 };
            }
            total
        }
        EncodeEnum::F64(data) => data.len() * 8,
        EncodeEnum::U64(data) => data.len() * 8,
    }
}

