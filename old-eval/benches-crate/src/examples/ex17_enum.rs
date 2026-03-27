// Example 17 — ENUM: Lifetime specialization (static vs borrowed)


// --- enum dispatch ---
pub enum ParseEnum<'a> {
    StaticStr(&'static str),
    BorrowedStr(&'a str),
    StaticBytes(&'static [u8]),
    BorrowedBytes(&'a [u8]),
}

#[inline(never)]
pub fn parse_enum(variant: &ParseEnum<'_>) -> usize {
    match variant {
        ParseEnum::StaticStr(_) => 42,
        ParseEnum::BorrowedStr(_) => 1,
        ParseEnum::StaticBytes(_) => 42,
        ParseEnum::BorrowedBytes(_) => 1,
    }
}

