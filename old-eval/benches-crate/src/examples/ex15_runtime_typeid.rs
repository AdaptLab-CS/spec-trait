// Example 15 — RUNTIME TYPEID: Byte conversion
use std::any::TypeId;


// --- runtime TypeId ---
#[inline(never)]
pub fn from_bytes_runtime_u8(data: &[u8]) -> Vec<u8> {
    if TypeId::of::<u8>() == TypeId::of::<u8>() {
        data.to_vec()
    } else { vec![] }
}

#[inline(never)]
pub fn from_bytes_runtime_char(data: &[u8]) -> Vec<char> {
    data.iter().map(|&b| b as char).collect()
}

#[inline(never)]
pub fn from_bytes_runtime_i32(data: &[u8]) -> Vec<i32> {
    data.chunks_exact(4)
        .map(|c| i32::from_le_bytes([c[0], c[1], c[2], c[3]]))
        .collect()
}

