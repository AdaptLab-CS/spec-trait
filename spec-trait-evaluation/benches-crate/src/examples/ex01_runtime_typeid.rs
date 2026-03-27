// Ex01 -- runtime TypeId dispatch: extend_from_slice
use std::any::TypeId;

#[inline(never)]
pub fn extend_runtime<T: Clone + Copy + 'static>(dst: &mut Vec<T>, src: &[T]) {
    if TypeId::of::<T>() == TypeId::of::<u8>() {
        let len = src.len();
        dst.reserve(len);
        unsafe {
            std::ptr::copy_nonoverlapping(src.as_ptr(), dst.as_mut_ptr().add(dst.len()), len);
            dst.set_len(dst.len() + len);
        }
    } else {
        for item in src {
            dst.push(item.clone());
        }
    }
}
