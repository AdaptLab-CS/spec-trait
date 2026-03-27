// Ex04 -- naive: sort — sort_unstable (no specialization)

#[inline(never)]
pub fn sort_naive<T: Ord>(data: &mut [T]) {
    data.sort_unstable();
}
