// Example 12 — RUNTIME TYPEID: Fill / memset
use std::any::TypeId;


// --- runtime TypeId ---
#[inline(never)]
pub fn fill_runtime_u8(buf: &mut [u8], val: u8) {
    if TypeId::of::<u8>() == TypeId::of::<u8>() {
        unsafe { std::ptr::write_bytes(buf.as_mut_ptr(), val, buf.len()); }
    } else {
        buf.fill(val);
    }
}

