// Ex08 -- spec: Count occurrences of a value, specialized for u8 (memchr)
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
        memchr::memchr_iter(*needle, data).count()
    }
}

pub fn count_val_u8(op: &CountValOp, data: &[u8], needle: &u8) -> usize {
    spec! { op.count_val(data, needle); CountValOp; [&[u8], &u8] }
}

pub fn _count_val_generic<T: PartialEq>(_op: &CountValOp, data: &[T], needle: &T) -> usize {
    data.iter().filter(|x| *x == needle).count()
}
