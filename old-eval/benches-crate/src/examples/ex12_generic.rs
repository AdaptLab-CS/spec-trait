// Example 12 — GENERIC: Fill / memset


// --- generic fallback ---
#[inline(never)]
pub fn fill_generic<T: Clone>(buf: &mut [T], val: T) {
    for slot in buf.iter_mut() {
        *slot = val.clone();
    }
}

// --- manual memset ---
#[inline(never)]
pub fn fill_memset_u8(buf: &mut [u8], val: u8) {
    unsafe { std::ptr::write_bytes(buf.as_mut_ptr(), val, buf.len()); }
}

#[inline(never)]
pub fn fill_stdlib_i32(buf: &mut [i32], val: i32) {
    buf.fill(val);
}
