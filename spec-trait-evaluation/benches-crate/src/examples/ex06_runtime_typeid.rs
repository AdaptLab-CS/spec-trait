// Ex06 -- runtime TypeId dispatch: dedup
use std::any::TypeId;
use std::collections::HashSet;
use std::hash::Hash;

#[inline(never)]
pub fn dedup_runtime<T: Clone + Eq + Hash + 'static>(data: &[T]) -> Vec<T> {
    if TypeId::of::<T>() == TypeId::of::<u8>() {
        let slice = unsafe {
            std::slice::from_raw_parts(data.as_ptr() as *const u8, data.len())
        };
        let mut seen = [false; 256];
        let mut out_bytes = Vec::new();
        for &b in slice {
            if !seen[b as usize] {
                seen[b as usize] = true;
                out_bytes.push(b);
            }
        }
        let mut out = Vec::with_capacity(out_bytes.len());
        unsafe {
            std::ptr::copy_nonoverlapping(out_bytes.as_ptr(), out.as_mut_ptr() as *mut u8, out_bytes.len());
            out.set_len(out_bytes.len());
        }
        out
    } else {
        let mut seen = HashSet::new();
        let mut out = Vec::new();
        for item in data {
            if seen.insert(item) {
                out.push(item.clone());
            }
        }
        out
    }
}
