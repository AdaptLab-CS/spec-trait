// Ex08 -- generic fallback: Count value (linear scan)

#[inline(never)]
pub fn count_val_generic_u8(data: &[u8], needle: u8) -> usize {
    data.iter().filter(|&&x| x == needle).count()
}
