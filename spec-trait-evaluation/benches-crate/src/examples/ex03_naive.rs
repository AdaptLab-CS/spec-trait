// Ex03 -- naive: contains — PartialOrd (no specialization)

pub struct Searcher;

pub trait NaiveContainsTrait<T> {
    fn in_range(&self, item: &T, lo: &T, hi: &T) -> bool;
}

impl<T: PartialOrd> NaiveContainsTrait<T> for Searcher {
    #[inline(never)]
    fn in_range(&self, item: &T, lo: &T, hi: &T) -> bool {
        item >= lo && item <= hi
    }
}

pub fn contains_naive(s: &Searcher, data: &[u8], lo: &u8, hi: &u8) -> usize {
    let mut count = 0;
    for item in data {
        if s.in_range(item, lo, hi) {
            count += 1;
        }
    }
    count
}
