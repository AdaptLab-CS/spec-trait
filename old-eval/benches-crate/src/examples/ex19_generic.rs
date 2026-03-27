// Example 19 — GENERIC: Multi-generic repeating (zip/combine)


// --- generic fallback ---
#[inline(never)]
pub fn combine_generic(xs: &[u8], ys: &[Vec<u8>]) -> usize {
    let mut total = xs.len();
    for v in ys {
        total += v.len();
    }
    total
}

// --- manual specialization ---
#[inline(never)]
pub fn combine_u8_vecu8(xs: &[u8], ys: &[Vec<u8>]) -> usize {
    let mut total = xs.len();
    for v in ys { total += v.len(); }
    total
}

#[inline(never)]
pub fn combine_i32_veci32(xs: &[i32], ys: &[Vec<i32>]) -> usize {
    let mut total = xs.len();
    for v in ys { total += v.len(); }
    total
}
