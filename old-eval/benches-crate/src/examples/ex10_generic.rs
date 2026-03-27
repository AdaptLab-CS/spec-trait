// Example 10 — GENERIC: Min/Max


// --- generic fallback using Ord ---
#[inline(never)]
pub fn minmax_generic_u8(data: &[u8]) -> (u8, u8) {
    let min = *data.iter().min().unwrap_or(&0);
    let max = *data.iter().max().unwrap_or(&0);
    (min, max)
}

#[inline(never)]
pub fn minmax_generic_i32(data: &[i32]) -> (i32, i32) {
    let min = *data.iter().min().unwrap_or(&0);
    let max = *data.iter().max().unwrap_or(&0);
    (min, max)
}
