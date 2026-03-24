// Example 9 — WITHOUT macros: Copy vs Clone\n\n// --- dyn dispatch (erased) ---
pub trait DynCopy {
    fn copy_dyn(&self) -> Vec<u8>;
    fn len(&self) -> usize;
}
impl DynCopy for [u8] {
    #[inline(never)]
    fn copy_dyn(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(self.len());
        unsafe {
            std::ptr::copy_nonoverlapping(self.as_ptr(), out.as_mut_ptr(), self.len());
            out.set_len(self.len());
        }
        out
    }
    fn len(&self) -> usize { <[u8]>::len(self) }
}

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
