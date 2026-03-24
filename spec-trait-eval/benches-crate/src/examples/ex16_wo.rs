// Example 16 — WITHOUT macros: Fill / memset
use std::any::TypeId;

// --- enum ---
pub enum FillEnum<'a> {
    U8(&'a mut [u8], u8),
    I32(&'a mut [i32], i32),
}
#[inline(never)]
pub fn fill_enum(variant: &mut FillEnum<'_>) {
    match variant {
        FillEnum::U8(buf, val) => unsafe { std::ptr::write_bytes(buf.as_mut_ptr(), *val, buf.len()); },
        FillEnum::I32(buf, val) => buf.fill(*val),
    }
}

// --- runtime TypeId ---
#[inline(never)]
pub fn fill_runtime_u8(buf: &mut [u8], val: u8) {
    if TypeId::of::<u8>() == TypeId::of::<u8>() {
        unsafe { std::ptr::write_bytes(buf.as_mut_ptr(), val, buf.len()); }
    } else {
        buf.fill(val);
    }
}

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
