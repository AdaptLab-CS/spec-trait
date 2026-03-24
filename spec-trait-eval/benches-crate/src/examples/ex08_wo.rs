// Example 8 — WITHOUT macros: Bulk serialization
use std::any::TypeId;

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

// --- runtime TypeId ---
#[inline(never)]
pub fn ser_bulk_runtime_u8(items: &[u8]) -> Vec<u8> {
    if TypeId::of::<u8>() == TypeId::of::<u8>() {
        let mut out = Vec::with_capacity(4 + items.len());
        out.extend_from_slice(&(items.len() as u32).to_le_bytes());
        out.extend_from_slice(items);
        out
    } else {
        vec![]
    }
}

#[inline(never)]
pub fn ser_bulk_runtime_i32(items: &[i32]) -> Vec<u8> {
    if TypeId::of::<i32>() == TypeId::of::<i32>() {
        let mut out = Vec::with_capacity(4 + items.len() * 4);
        out.extend_from_slice(&(items.len() as u32).to_le_bytes());
        for v in items { out.extend_from_slice(&v.to_le_bytes()); }
        out
    } else {
        vec![]
    }
}

// --- generic fallback ---
#[inline(never)]
pub fn ser_bulk_generic<T: std::fmt::Debug>(items: &[T]) -> Vec<u8> {
    let mut out = Vec::new();
    for item in items {
        let s = format!("{:?}", item);
        out.extend_from_slice(&(s.len() as u32).to_le_bytes());
        out.extend_from_slice(s.as_bytes());
    }
    out
}
