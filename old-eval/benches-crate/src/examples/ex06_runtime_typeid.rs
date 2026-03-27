// Example 06 — RUNTIME TYPEID: Dedup
use std::any::TypeId;


// --- runtime TypeId ---
#[inline(never)]
pub fn dedup_runtime_u8(data: &[u8]) -> Vec<u8> {
    if TypeId::of::<u8>() == TypeId::of::<u8>() {
        let mut out = Vec::with_capacity(data.len());
        let mut prev: Option<u8> = None;
        for &b in data { if prev != Some(b) { out.push(b); prev = Some(b); } }
        out
    } else {
        data.to_vec()
    }
}

