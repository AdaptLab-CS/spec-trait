// Example 09 — ENUM: Display-based formatting


// --- enum ---
pub enum FmtEnum<'a> {
    I32(&'a [i32]),
    F64(&'a [f64]),
    Str(&'a [String]),
}
#[inline(never)]
pub fn fmt_enum_all(variant: &FmtEnum<'_>) -> String {
    match variant {
        FmtEnum::I32(items) => {
            let mut buf = itoa::Buffer::new();
            items.iter().map(|x| buf.format(*x).to_owned()).collect::<Vec<_>>().join(", ")
        }
        FmtEnum::F64(items) => {
            let mut buf = ryu::Buffer::new();
            items.iter().map(|x| buf.format(*x).to_owned()).collect::<Vec<_>>().join(", ")
        }
        FmtEnum::Str(items) => items.join(", "),
    }
}

