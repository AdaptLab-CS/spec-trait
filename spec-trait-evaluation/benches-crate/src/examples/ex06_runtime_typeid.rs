// Ex06 — runtime TypeId dispatch: Bulk serialize
use std::any::TypeId;

#[inline(never)]
pub fn ser_bulk_runtime_u8(data: &[u8]) -> Vec<u8> {
    if TypeId::of::<u8>() == TypeId::of::<u8>() {
        data.to_vec()
    } else {
        let mut out = Vec::new();
        for item in data {
            out.extend_from_slice(format!("{}", item).as_bytes());
            out.push(b',');
        }
        out
    }
}
