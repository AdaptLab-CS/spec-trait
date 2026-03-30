// Ex08 -- naive: count_value — PartialEq (no specialization)

pub struct CountValOp;

pub trait NaiveMatchesTrait<T> {
    fn matches(&self, item: &T, needle: &T) -> bool;
}

impl<T: PartialEq> NaiveMatchesTrait<T> for CountValOp {
    #[inline(never)]
    fn matches(&self, item: &T, needle: &T) -> bool {
        item == needle
    }
}

pub fn count_val_naive(op: &CountValOp, data: &[u8], needle: &u8) -> usize {
    let mut count = 0;
    for item in data {
        if op.matches(item, needle) {
            count += 1;
        }
    }
    count
}
