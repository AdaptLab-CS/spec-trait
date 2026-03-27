// Ex01 -- spec: extend_from_slice — Copy memcpy vs Clone loop
use spec_trait_macro::{spec, when};

pub struct Extender;

pub trait ExtendTrait<T> {
    fn extend_slice(&self, dst: &mut Vec<T>, src: &[T]);
}

impl<T: Clone> ExtendTrait<T> for Extender {
    #[inline(never)]
    fn extend_slice(&self, dst: &mut Vec<T>, src: &[T]) {
        for item in src {
            dst.push(item.clone());
        }
    }
}

pub fn extend_u8(e: &Extender, dst: &mut Vec<u8>, src: &[u8]) {
    spec! { e.extend_slice(dst, src); Extender; [&mut Vec<u8>, &[u8]]; u8: Copy }
}

#[when(T: Copy)]
impl<T> ExtendTrait<T> for Extender {
    #[inline(never)]
    fn extend_slice(&self, dst: &mut Vec<T>, src: &[T]) {
        let len = src.len();
        dst.reserve(len);
        unsafe {
            std::ptr::copy_nonoverlapping(src.as_ptr(), dst.as_mut_ptr().add(dst.len()), len);
            dst.set_len(dst.len() + len);
        }
    }
}


