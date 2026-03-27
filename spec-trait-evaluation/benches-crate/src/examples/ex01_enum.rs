// Ex01 -- enum dispatch: extend_from_slice

pub enum ExtendEnum<'a, T> {
    U8(&'a [u8]),
    Generic(&'a [T]),
}

pub trait EnumExtendTrait<T> {
    fn extend_to(&self, dst: &mut Vec<T>);
}

impl<T: Clone> EnumExtendTrait<T> for ExtendEnum<'_, T> {
    #[inline(never)]
    fn extend_to(&self, dst: &mut Vec<T>) {
        match self {
            ExtendEnum::U8(src) => {
                // Fast path: bulk memcpy (only valid when T = u8, enforced by entry fn)
                let len = src.len();
                dst.reserve(len);
                unsafe {
                    std::ptr::copy_nonoverlapping(
                        src.as_ptr(),
                        dst.as_mut_ptr().add(dst.len()) as *mut u8,
                        len,
                    );
                    dst.set_len(dst.len() + len);
                }
            }
            ExtendEnum::Generic(src) => {
                for item in *src {
                    dst.push(item.clone());
                }
            }
        }
    }
}

pub fn extend_enum(e: &ExtendEnum<u8>, dst: &mut Vec<u8>) {
    e.extend_to(dst);
}
