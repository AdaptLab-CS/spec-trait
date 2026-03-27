// Example 05 — GENERIC: Copy vs Clone\n\n// --- dyn dispatch (erased) ---


// --- generic clone fallback ---
#[inline(never)]
pub fn copy_generic<T: Clone>(data: &[T]) -> Vec<T> {
    data.to_vec()
}

// --- manual Copy fast path ---
#[inline(never)]
pub fn copy_memcpy_i32(data: &[i32]) -> Vec<i32> {
    let mut out = Vec::with_capacity(data.len());
    unsafe {
        std::ptr::copy_nonoverlapping(data.as_ptr(), out.as_mut_ptr(), data.len());
        out.set_len(data.len());
    }
    out
}

#[inline(never)]
pub fn copy_memcpy_u8(data: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(data.len());
    unsafe {
        std::ptr::copy_nonoverlapping(data.as_ptr(), out.as_mut_ptr(), data.len());
        out.set_len(data.len());
    }
    out
}
