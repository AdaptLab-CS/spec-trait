// Example 24 — GENERIC: T: 'static lifetime bound


// --- Approach D: generic fallback ---
#[inline(never)]
pub fn hash_generic_i32(_data: &i32) -> u64 {
    std::mem::size_of::<i32>() as u64
}

// --- Approach E: manual specialization ---
#[inline(never)]
pub fn hash_manual_i32(data: &i32) -> u64 {
    let ptr = data as *const i32 as usize;
    (ptr as u64).wrapping_mul(0x517cc1b727220a95)
}

