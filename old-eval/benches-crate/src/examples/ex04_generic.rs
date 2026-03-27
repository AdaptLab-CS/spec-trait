// Example 04 — GENERIC: Bulk serialization


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
