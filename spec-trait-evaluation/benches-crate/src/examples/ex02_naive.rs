// Ex02 -- naive: to_bytes — Display formatting (no specialization)
use std::fmt::Display;

pub struct Serializer;

pub trait NaiveToByteTrait<T> {
    fn to_bytes(&self, data: &[T]) -> Vec<u8>;
}

impl<T: Display> NaiveToByteTrait<T> for Serializer {
    #[inline(never)]
    fn to_bytes(&self, data: &[T]) -> Vec<u8> {
        let mut result = Vec::new();
        for item in data {
            result.extend_from_slice(format!("{}", item).as_bytes());
        }
        result
    }
}

pub fn to_bytes_naive(s: &Serializer, data: &[u8]) -> Vec<u8> {
    s.to_bytes(data)
}
