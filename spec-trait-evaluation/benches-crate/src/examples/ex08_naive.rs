// Ex08 -- naive: count_value — filter (no specialization)

#[inline(never)]
pub fn count_val_naive<T: PartialEq>(data: &[T], needle: &T) -> usize {
    data.iter().filter(|x| *x == needle).count()
}
