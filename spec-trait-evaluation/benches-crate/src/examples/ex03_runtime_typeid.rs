// Ex03 — runtime TypeId dispatch: Contains search
use std::any::TypeId;

#[inline(never)]
pub fn contains_runtime_u8(data: &[u8], needle: &u8) -> bool {
    if TypeId::of::<u8>() == TypeId::of::<u8>() {
        memchr::memchr(*needle, data).is_some()
    } else {
        data.iter().any(|x| x == needle)
    }
}
