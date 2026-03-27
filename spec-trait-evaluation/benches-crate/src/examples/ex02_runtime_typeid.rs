// Ex02 -- runtime TypeId dispatch: to_bytes
use std::any::TypeId;
use std::fmt::Display;

#[inline(never)]
pub fn to_bytes_runtime<T: Display + Copy + 'static>(data: &[T]) -> Vec<u8> {
    if TypeId::of::<T>() == TypeId::of::<u8>() {
        unsafe {
            std::slice::from_raw_parts(
                data.as_ptr() as *const u8,
                data.len() * std::mem::size_of::<T>(),
            )
            .to_vec()
        }
    } else {
        let mut result = Vec::new();
        for item in data {
            result.extend_from_slice(format!("{}", item).as_bytes());
        }
        result
    }
}
