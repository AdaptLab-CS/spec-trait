// Ex06 -- enum dispatch: dedup
use std::collections::HashSet;
use std::hash::Hash;

pub enum DedupEnum<'a, T> {
    U8(&'a [u8]),
    Generic(&'a [T]),
}

pub trait EnumDedupTrait<T> {
    fn dedup(&self) -> Vec<T>;
}

impl<T: Clone + Eq + Hash> EnumDedupTrait<T> for DedupEnum<'_, T> {
    #[inline(never)]
    fn dedup(&self) -> Vec<T> {
        match self {
            DedupEnum::U8(data) => {
                let mut seen = [false; 256];
                let mut out_bytes: Vec<u8> = Vec::new();
                for &b in *data {
                    if !seen[b as usize] {
                        seen[b as usize] = true;
                        out_bytes.push(b);
                    }
                }
                // Safety: only reachable when T = u8 (enforced by entry fn)
                let (ptr, len, cap) = (out_bytes.as_mut_ptr(), out_bytes.len(), out_bytes.capacity());
                std::mem::forget(out_bytes);
                unsafe { Vec::from_raw_parts(ptr as *mut T, len, cap) }
            }
            DedupEnum::Generic(data) => {
                let mut seen = HashSet::new();
                let mut out = Vec::new();
                for item in *data {
                    if seen.insert(item) {
                        out.push(item.clone());
                    }
                }
                out
            }
        }
    }
}

pub fn dedup_enum(e: &DedupEnum<u8>) -> Vec<u8> {
    e.dedup()
}
