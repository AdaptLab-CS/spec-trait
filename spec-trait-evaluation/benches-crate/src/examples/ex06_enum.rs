// Ex06 -- enum dispatch: transform
use std::hash::Hash;

pub enum DedupEnum<'a, T> {
    U8(&'a [u8]),
    Generic(&'a [T]),
}

#[inline(never)]
fn transform_u8_direct(item: &u8) -> u8 {
    *item ^ 0x55
}

pub trait EnumDedupTrait<T> {
    fn dedup(&self) -> Vec<T>;
}

impl<T: Clone + Eq + Hash> EnumDedupTrait<T> for DedupEnum<'_, T> {
    #[inline(never)]
    fn dedup(&self) -> Vec<T> {
        match self {
            DedupEnum::U8(data) => {
                let mut out_bytes: Vec<u8> = Vec::with_capacity(data.len());
                for &b in *data {
                    out_bytes.push(transform_u8_direct(&b));
                }
                let (ptr, len, cap) = (out_bytes.as_mut_ptr(), out_bytes.len(), out_bytes.capacity());
                std::mem::forget(out_bytes);
                unsafe { Vec::from_raw_parts(ptr as *mut T, len, cap) }
            }
            DedupEnum::Generic(data) => {
                data.to_vec()
            }
        }
    }
}

pub fn dedup_enum(e: &DedupEnum<u8>) -> Vec<u8> {
    e.dedup()
}
