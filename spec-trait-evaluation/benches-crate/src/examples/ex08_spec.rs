// Ex08 -- spec: count_value — memchr SIMD vs filter().count()
use spec_trait_macro::{spec, when};

pub struct CountValOp;

pub trait CountValTrait<T> {
    fn count_val(&self, data: &[T], needle: &T) -> usize;
}

impl<T: PartialEq> CountValTrait<T> for CountValOp {
    #[inline(never)]
    fn count_val(&self, data: &[T], needle: &T) -> usize {
        data.iter().filter(|x| *x == needle).count()
    }
}

#[when(T = u8)]
impl<T> CountValTrait<T> for CountValOp {
    #[inline(never)]
    fn count_val(&self, data: &[T], needle: &T) -> usize {
        let byte_slice = unsafe {
            std::slice::from_raw_parts(data.as_ptr() as *const u8, data.len())
        };
        let n = unsafe { *(needle as *const T as *const u8) };
        memchr::memchr_iter(n, byte_slice).count()
    }
}

pub fn count_val_u8(op: &CountValOp, data: &[u8], needle: &u8) -> usize {
    spec! { op.count_val(data, needle); CountValOp; [&[u8], &u8] }
}
