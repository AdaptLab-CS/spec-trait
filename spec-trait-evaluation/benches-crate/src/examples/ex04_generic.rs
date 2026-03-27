// Ex04 — generic fallback: Sort (stdlib sort_unstable)
#[inline(never)]
pub fn sort_generic_u8(data: &mut [u8]) {
    data.sort_unstable();
}
