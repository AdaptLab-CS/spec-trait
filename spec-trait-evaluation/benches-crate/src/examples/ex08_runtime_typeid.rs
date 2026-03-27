// Ex08 -- runtime TypeId dispatch: count_value
use std::any::TypeId;

pub struct CountValOp;

pub trait RtCountValTrait<T> {
    fn count_val(&self, data: &[T], needle: &T) -> usize;
}

impl<T: PartialEq + 'static> RtCountValTrait<T> for CountValOp {
    #[inline(never)]
    fn count_val(&self, data: &[T], needle: &T) -> usize {
        if TypeId::of::<T>() == TypeId::of::<u8>() {
            let byte_slice = unsafe {
                std::slice::from_raw_parts(data.as_ptr() as *const u8, data.len())
            };
            let n = unsafe { *(needle as *const T as *const u8) };
            memchr::memchr_iter(n, byte_slice).count()
        } else {
            data.iter().filter(|x| *x == needle).count()
        }
    }
}

pub fn count_val_runtime(op: &CountValOp, data: &[u8], needle: &u8) -> usize {
    op.count_val(data, needle)
}
