// Ex08 -- spec: count_value — direct byte match for u8 vs PartialEq
use spec_trait_macro::{spec, when};

pub struct CountValOp;

pub trait MatchesTrait<T> {
    fn matches(&self, item: &T, needle: &T) -> bool;
}

impl<T: PartialEq> MatchesTrait<T> for CountValOp {
    #[inline(never)]
    fn matches(&self, item: &T, needle: &T) -> bool {
        item == needle
    }
}

#[when(T = u8)]
impl<T> MatchesTrait<T> for CountValOp {
    #[inline(never)]
    fn matches(&self, item: &T, needle: &T) -> bool {
        unsafe { *(item as *const T as *const u8) == *(needle as *const T as *const u8) }
    }
}

#[inline(always)]
fn matches_spec(op: &CountValOp, item: &u8, needle: &u8) -> bool {
    spec! { op.matches(item, needle); CountValOp; [&u8, &u8] }
}

pub fn count_val_u8(op: &CountValOp, data: &[u8], needle: &u8) -> usize {
    let mut count = 0;
    for item in data {
        if matches_spec(op, item, needle) {
            count += 1;
        }
    }
    count
}
