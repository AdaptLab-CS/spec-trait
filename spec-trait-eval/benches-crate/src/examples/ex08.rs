// Example 8 — WITH macros: Vec<u8> length-prefixed serialization
use spec_trait_macro::{spec, when};

pub struct BulkSerializer;

pub trait BulkSerTrait<T> {
    fn ser_bulk(&self, items: &[T]) -> Vec<u8>;
}

impl<T: std::fmt::Debug> BulkSerTrait<T> for BulkSerializer {
    #[inline(never)]
    fn ser_bulk(&self, items: &[T]) -> Vec<u8> {
        let mut out = Vec::new();
        for item in items {
            let s = format!("{:?}", item);
            out.extend_from_slice(&(s.len() as u32).to_le_bytes());
            out.extend_from_slice(s.as_bytes());
        }
        out
    }
}

#[when(T = u8)]
impl<T> BulkSerTrait<T> for BulkSerializer {
    #[inline(never)]
    fn ser_bulk(&self, items: &[T]) -> Vec<u8> {
        let mut out = Vec::with_capacity(4 + items.len());
        out.extend_from_slice(&(items.len() as u32).to_le_bytes());
        out.extend_from_slice(items);
        out
    }
}

#[when(T = i32)]
impl<T> BulkSerTrait<T> for BulkSerializer {
    #[inline(never)]
    fn ser_bulk(&self, items: &[T]) -> Vec<u8> {
        let mut out = Vec::with_capacity(4 + items.len() * 4);
        out.extend_from_slice(&(items.len() as u32).to_le_bytes());
        for v in items {
            out.extend_from_slice(&v.to_le_bytes());
        }
        out
    }
}

pub fn ser_bulk_u8(s: &BulkSerializer, items: &[u8]) -> Vec<u8> {
    spec! { s.ser_bulk(items); BulkSerializer; [u8] }
}

pub fn ser_bulk_i32(s: &BulkSerializer, items: &[i32]) -> Vec<u8> {
    spec! { s.ser_bulk(items); BulkSerializer; [i32] }
}

pub fn ser_bulk_generic<T: std::fmt::Debug>(s: &BulkSerializer, items: &[T]) -> Vec<u8> {
    BulkSerTrait::ser_bulk(s, items)
}
