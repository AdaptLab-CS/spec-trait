// Example 15 — ENUM: Byte conversion


// --- enum ---
pub enum FromBytesEnum<'a> {
    U8(&'a [u8]),
    Char(&'a [u8]),
    I32(&'a [u8]),
}
#[inline(never)]
pub fn frombytes_enum_u8(variant: &FromBytesEnum<'_>) -> Vec<u8> {
    match variant {
        FromBytesEnum::U8(data) => data.to_vec(),
        _ => vec![],
    }
}

#[inline(never)]
pub fn frombytes_enum_char(variant: &FromBytesEnum<'_>) -> Vec<char> {
    match variant {
        FromBytesEnum::Char(data) => data.iter().map(|&b| b as char).collect(),
        _ => vec![],
    }
}

#[inline(never)]
pub fn frombytes_enum_i32(variant: &FromBytesEnum<'_>) -> Vec<i32> {
    match variant {
        FromBytesEnum::I32(data) => {
            data.chunks_exact(4)
                .map(|c| i32::from_le_bytes([c[0], c[1], c[2], c[3]]))
                .collect()
        }
        _ => vec![],
    }
}

