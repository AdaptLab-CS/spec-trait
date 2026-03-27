// Example 06 — ENUM: Dedup


// --- enum dispatch ---
pub enum DedupEnum<'a> {
    U8(&'a [u8]),
    I32(&'a [i32]),
}
#[inline(never)]
pub fn dedup_enum_u8(variant: &DedupEnum<'_>) -> Vec<u8> {
    match variant {
        DedupEnum::U8(data) => {
            let mut out = Vec::with_capacity(data.len());
            let mut prev: Option<u8> = None;
            for &b in *data {
                if prev != Some(b) { out.push(b); prev = Some(b); }
            }
            out
        }
        _ => vec![],
    }
}

#[inline(never)]
pub fn dedup_enum_i32(variant: &DedupEnum<'_>) -> Vec<i32> {
    match variant {
        DedupEnum::I32(data) => {
            let mut out = Vec::with_capacity(data.len());
            let mut prev: Option<i32> = None;
            for &v in *data { if prev != Some(v) { out.push(v); prev = Some(v); } }
            out
        }
        _ => vec![],
    }
}

