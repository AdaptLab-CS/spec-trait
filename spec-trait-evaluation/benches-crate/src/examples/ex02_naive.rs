// Ex02 -- naive: to_bytes — Display formatting (no specialization)
use std::fmt::Display;

#[inline(never)]
pub fn to_bytes_naive<T: Display>(data: &[T]) -> Vec<u8> {
    let mut result = Vec::new();
    for item in data {
        result.extend_from_slice(format!("{}", item).as_bytes());
    }
    result
}
