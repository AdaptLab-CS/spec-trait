// Example 04 — ENUM: Bulk serialization


// --- enum dispatch ---
pub enum BulkSerEnum<'a> {
    U8(&'a [u8]),
    I32(&'a [i32]),
}
#[inline(never)]
pub fn bulkser_enum_ser_bulk(variant: &BulkSerEnum<'_>) -> Vec<u8> {
    match variant {
        BulkSerEnum::U8(items) => {
            let mut out = Vec::with_capacity(4 + items.len());
            out.extend_from_slice(&(items.len() as u32).to_le_bytes());
            out.extend_from_slice(items);
            out
        }
        BulkSerEnum::I32(items) => {
            let mut out = Vec::with_capacity(4 + items.len() * 4);
            out.extend_from_slice(&(items.len() as u32).to_le_bytes());
            for v in *items { out.extend_from_slice(&v.to_le_bytes()); }
            out
        }
    }
}

