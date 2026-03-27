// Ex08 -- naive: count_value — filter (no specialization)

pub struct CountValOp;

pub trait NaiveCountValTrait<T> {
    fn count_val(&self, data: &[T], needle: &T) -> usize;
}

impl<T: PartialEq> NaiveCountValTrait<T> for CountValOp {
    #[inline(never)]
    fn count_val(&self, data: &[T], needle: &T) -> usize {
        data.iter().filter(|x| *x == needle).count()
    }
}

pub fn count_val_naive(op: &CountValOp, data: &[u8], needle: &u8) -> usize {
    op.count_val(data, needle)
}
