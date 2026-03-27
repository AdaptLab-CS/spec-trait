// Ex08 -- runtime TypeId dispatch: Count value
use std::any::TypeId;

#[inline(never)]
pub fn count_val_runtime_u8(data: &[u8], needle: u8) -> usize {
    if TypeId::of::<u8>() == TypeId::of::<u8>() {
        memchr::memchr_iter(needle, data).count()
    } else {
        data.iter().filter(|&&x| x == needle).count()
    }
}
