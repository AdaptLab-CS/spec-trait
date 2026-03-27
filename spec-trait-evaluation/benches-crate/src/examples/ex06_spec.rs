// Ex06 — spec!: Bulk serialize (u8 zero-copy vs generic Display)
use spec_trait_macro::{spec, when};

pub struct BulkSerializer;

pub trait BulkSerTrait<T> {
    fn ser_bulk(&self, data: &[T]) -> Vec<u8>;
}

impl<T: std::fmt::Display> BulkSerTrait<T> for BulkSerializer {
    #[inline(never)]
    fn ser_bulk(&self, data: &[T]) -> Vec<u8> {
        let mut out = Vec::new();
        for item in data {
            out.extend_from_slice(format!("{}", item).as_bytes());
            out.push(b',');
        }
        out
    }
}

#[when(T = u8)]
impl<T> BulkSerTrait<T> for BulkSerializer {
    #[inline(never)]
    fn ser_bulk(&self, data: &[T]) -> Vec<u8> {
        // Zero-copy: the data IS the serialized form
        data.to_vec()
    }
}

pub fn ser_bulk_u8(bs: &BulkSerializer, data: &[u8]) -> Vec<u8> {
    spec! { bs.ser_bulk(data); BulkSerializer; [&[u8]] }
}

pub fn ser_bulk_generic<T: std::fmt::Display>(_bs: &BulkSerializer, data: &[T]) -> Vec<u8> {
    let mut out = Vec::new();
    for item in data {
        out.extend_from_slice(format!("{}", item).as_bytes());
        out.push(b',');
    }
    out
}
