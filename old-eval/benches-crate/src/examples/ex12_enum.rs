// Example 12 — ENUM: Fill / memset


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

