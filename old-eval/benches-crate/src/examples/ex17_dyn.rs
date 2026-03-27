// Example 17 — DYN: Lifetime specialization (static vs borrowed)


// --- dyn dispatch ---
pub trait DynParse {
    fn parse_dyn(&self) -> usize;
}

pub struct DynStaticStr(pub &'static str);
impl DynParse for DynStaticStr {
    #[inline(never)]
    fn parse_dyn(&self) -> usize { 42 }
}

pub struct DynBorrowedStr<'a>(pub &'a str);
impl DynParse for DynBorrowedStr<'_> {
    #[inline(never)]
    fn parse_dyn(&self) -> usize { 1 }
}

