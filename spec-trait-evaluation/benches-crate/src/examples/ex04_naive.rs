// Ex04 -- naive: sort — generic Ord comparison (no specialization)

pub struct Sorter;

pub trait NaiveCmpTrait<T> {
    fn compare(&self, a: &T, b: &T) -> std::cmp::Ordering;
}

impl<T: Ord> NaiveCmpTrait<T> for Sorter {
    #[inline(never)]
    fn compare(&self, a: &T, b: &T) -> std::cmp::Ordering {
        a.cmp(b)
    }
}

pub fn sort_naive(s: &Sorter, data: &mut [u8]) {
    data.sort_unstable_by(|a, b| s.compare(a, b));
}
