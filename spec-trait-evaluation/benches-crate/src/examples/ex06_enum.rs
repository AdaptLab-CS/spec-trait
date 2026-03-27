// Ex06 — enum dispatch: Bulk serialize
pub enum BulkSerEnum<'a> {
    U8(&'a [u8]),
    I32(&'a [i32]),
}

#[inline(never)]
pub fn bulkser_enum(variant: &BulkSerEnum<'_>) -> Vec<u8> {
    match variant {
        BulkSerEnum::U8(data) => data.to_vec(),
        BulkSerEnum::I32(data) => {
            let mut out = Vec::new();
            for item in *data {
                out.extend_from_slice(format!("{}", item).as_bytes());
                out.push(b',');
            }
            out
        }
    }
}
