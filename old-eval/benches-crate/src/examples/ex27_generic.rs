// Example 27 — GENERIC: Many specializations (8+ types)


// --- Approach C: generic fallback ---
#[inline(never)]
pub fn encode_generic_u8(data: &[u8]) -> usize {
    data.len() * std::mem::size_of::<u8>()
}

#[inline(never)]
pub fn encode_generic_i32(data: &[i32]) -> usize {
    data.len() * std::mem::size_of::<i32>()
}

// --- Approach D: manual specialization ---
#[inline(never)]
pub fn encode_manual_u8(data: &[u8]) -> usize {
    data.len()
}

#[inline(never)]
pub fn encode_manual_i32(data: &[i32]) -> usize {
    let mut total = 0usize;
    for &v in data {
        let uv = v as u32 as usize;
        total += if uv < 128 { 1 } else if uv < 16384 { 2 } else { 4 };
    }
    total
}

