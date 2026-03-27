// Ex06 — generic fallback: Bulk serialize via Display
#[inline(never)]
pub fn ser_bulk_generic_u8(data: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    for item in data {
        out.extend_from_slice(format!("{}", item).as_bytes());
        out.push(b',');
    }
    out
}
