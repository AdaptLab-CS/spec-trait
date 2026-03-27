// Ex01 -- runtime TypeId dispatch: extend_from_slice
use std::any::TypeId;

pub struct Extender;

pub trait RtExtendTrait<T> {
    fn extend_slice(&self, dst: &mut Vec<T>, src: &[T]);
}

impl<T: Clone + Copy + 'static> RtExtendTrait<T> for Extender {
    #[inline(never)]
    fn extend_slice(&self, dst: &mut Vec<T>, src: &[T]) {
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
}

pub fn extend_runtime(e: &Extender, dst: &mut Vec<u8>, src: &[u8]) {
    e.extend_slice(dst, src);
}
