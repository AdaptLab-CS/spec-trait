// Ex04 -- naive: sort — sort_unstable (no specialization)

pub struct Sorter;

pub trait NaiveSortTrait<T> {
    fn sort(&self, data: &mut [T]);
}

impl<T: Ord> NaiveSortTrait<T> for Sorter {
    #[inline(never)]
    fn sort(&self, data: &mut [T]) {
        data.sort_unstable();
    }
}

pub fn sort_naive(s: &Sorter, data: &mut [u8]) {
    s.sort(data);
}
