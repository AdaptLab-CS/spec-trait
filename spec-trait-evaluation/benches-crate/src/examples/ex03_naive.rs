// Ex03 -- naive: contains — linear scan (no specialization)

#[inline(never)]
pub fn contains_naive<T: PartialEq>(data: &[T], needle: &T) -> bool {
    data.iter().any(|x| x == needle)
}
