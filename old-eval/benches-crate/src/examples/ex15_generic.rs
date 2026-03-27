// Example 15 — GENERIC: Byte conversion


// --- generic fallback ---
#[inline(never)]
pub fn from_bytes_generic_u8(data: &[u8]) -> Vec<u8> {
    data.to_vec()
}
