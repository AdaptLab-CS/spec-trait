// Example 04 — RUNTIME TYPEID: Bulk serialization
use std::any::TypeId;


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

