// Ex01 -- enum dispatch: extend_from_slice

pub enum ExtendEnum<'a> {
    U8(&'a [u8]),
}


#[inline(never)]
pub fn extend_enum(e: &ExtendEnum<'_>, dst: &mut Vec<u8>) {
    match e {
        ExtendEnum::U8(src) => {
            let len = src.len();
            dst.reserve(len);
            unsafe {
                std::ptr::copy_nonoverlapping(src.as_ptr(), dst.as_mut_ptr().add(dst.len()), len);
                dst.set_len(dst.len() + len);
            }
        }
    }
}
