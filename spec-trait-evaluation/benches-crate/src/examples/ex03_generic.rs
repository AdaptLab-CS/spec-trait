// Ex03 — generic fallback: Contains search (linear scan, no memchr)
#[inline(never)]
pub fn contains_generic_u8(data: &[u8], needle: &u8) -> bool {
    data.iter().any(|x| x == needle)
}

#[inline(never)]
pub fn contains_generic_i32(data: &[i32], needle: &i32) -> bool {
    data.iter().any(|x| x == needle)
}
