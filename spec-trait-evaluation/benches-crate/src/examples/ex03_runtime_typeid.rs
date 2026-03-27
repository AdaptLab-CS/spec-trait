// Ex03 -- runtime TypeId dispatch: contains
use std::any::TypeId;

#[inline(never)]
pub fn contains_runtime<T: PartialEq + 'static>(data: &[T], needle: &T) -> bool {
    if TypeId::of::<T>() == TypeId::of::<u8>() {
        let byte_slice = unsafe {
            std::slice::from_raw_parts(data.as_ptr() as *const u8, data.len())
        };
        let n = unsafe { *(needle as *const T as *const u8) };
        memchr::memchr(n, byte_slice).is_some()
    } else {
        data.iter().any(|x| x == needle)
    }
}
