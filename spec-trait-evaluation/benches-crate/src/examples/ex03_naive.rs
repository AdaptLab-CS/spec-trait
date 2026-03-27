// Ex03 -- naive: contains — linear scan (no specialization)

pub struct Searcher;

pub trait NaiveContainsTrait<T> {
    fn contains(&self, data: &[T], needle: &T) -> bool;
}

impl<T: PartialEq> NaiveContainsTrait<T> for Searcher {
    #[inline(never)]
    fn contains(&self, data: &[T], needle: &T) -> bool {
        data.iter().any(|x| x == needle)
    }
}

pub fn contains_naive(s: &Searcher, data: &[u8], needle: &u8) -> bool {
    s.contains(data, needle)
}
