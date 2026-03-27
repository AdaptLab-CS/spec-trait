// Example 20 — GENERIC: Wildcard and complex type patterns


// --- generic fallback ---
#[inline(never)]
pub fn measure_generic_vec_i32(data: &Vec<i32>) -> usize {
    std::mem::size_of::<Vec<i32>>() + data.len() * std::mem::size_of::<i32>()
}

// --- manual specialization ---
#[inline(never)]
pub fn measure_vec_i32(_data: &Vec<i32>) -> usize {
    std::mem::size_of::<Vec<i32>>() + 24
}

#[inline(never)]
pub fn measure_vec_u8(_data: &Vec<u8>) -> usize {
    std::mem::size_of::<Vec<u8>>() + 24
}

