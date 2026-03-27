// Example 34 — GENERIC: 'static + Copy compound bound


// --- generic fallback ---
#[inline(never)]
pub fn cached_size_generic_i32(data: &[i32]) -> usize {
    data.len()
}

#[inline(never)]
pub fn cached_size_generic_u8(data: &[u8]) -> usize {
    data.len()
}

