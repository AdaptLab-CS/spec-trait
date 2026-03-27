// Example 05 — ENUM: Copy vs Clone\n\n// --- dyn dispatch (erased) ---


// --- enum dispatch ---
pub enum CopyEnum<'a> {
    U8(&'a [u8]),
    I32(&'a [i32]),
    F64(&'a [f64]),
}
#[inline(never)]
pub fn copy_enum_u8(variant: &CopyEnum<'_>) -> Vec<u8> {
    match variant {
        CopyEnum::U8(d) => {
            let mut out = Vec::with_capacity(d.len());
            unsafe {
                std::ptr::copy_nonoverlapping(d.as_ptr(), out.as_mut_ptr(), d.len());
                out.set_len(d.len());
            }
            out
        }
        _ => vec![],
    }
}

#[inline(never)]
pub fn copy_enum_i32(variant: &CopyEnum<'_>) -> Vec<i32> {
    match variant {
        CopyEnum::I32(d) => {
            let mut out = Vec::with_capacity(d.len());
            unsafe {
                std::ptr::copy_nonoverlapping(d.as_ptr(), out.as_mut_ptr(), d.len());
                out.set_len(d.len());
            }
            out
        }
        _ => vec![],
    }
}

