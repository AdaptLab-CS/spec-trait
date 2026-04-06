// Ex08 -- runtime TypeId dispatch: count_value
use std::any::TypeId;

pub struct CountValOp;

pub trait RtMatchesTrait<T> {
    fn matches(&self, item: &T, needle: &T) -> bool;
}

impl<T: PartialEq + 'static> RtMatchesTrait<T> for CountValOp {
    #[inline(never)]
    fn matches(&self, item: &T, needle: &T) -> bool {
        if TypeId::of::<T>() == TypeId::of::<u8>() {
            unsafe { *(item as *const T as *const u8) == *(needle as *const T as *const u8) }
        } else {
            item == needle
        }
    }
}

pub fn count_val_runtime(op: &CountValOp, data: &[u8], needle: &u8) -> usize {
    let mut count = 0;
    for item in data {
        if op.matches(item, needle) {
            count += 1;
        }
    }
    count
}
