// Ex01 -- naive: extend_from_slice — clone loop (no specialization)

#[inline(never)]
pub fn extend_naive<T: Clone>(dst: &mut Vec<T>, src: &[T]) {
    for item in src {
        dst.push(item.clone());
    }
}
