// Example 13 — WITHOUT macros: Display-based formatting
use std::any::TypeId;

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

// --- runtime TypeId ---
#[inline(never)]
pub fn fmt_runtime<T: std::fmt::Debug + 'static>(items: &[T]) -> String {
    let tid = TypeId::of::<T>();
    if tid == TypeId::of::<i32>() {
        let data = unsafe { &*(items as *const [T] as *const [i32]) };
        let mut buf = itoa::Buffer::new();
        data.iter().map(|x| buf.format(*x).to_owned()).collect::<Vec<_>>().join(", ")
    } else if tid == TypeId::of::<f64>() {
        let data = unsafe { &*(items as *const [T] as *const [f64]) };
        let mut buf = ryu::Buffer::new();
        data.iter().map(|x| buf.format(*x).to_owned()).collect::<Vec<_>>().join(", ")
    } else {
        items.iter().map(|x| format!("{:?}", x)).collect::<Vec<_>>().join(", ")
    }
}

// --- generic fallback ---
#[inline(never)]
pub fn fmt_generic<T: std::fmt::Debug>(items: &[T]) -> String {
    items.iter().map(|x| format!("{:?}", x)).collect::<Vec<_>>().join(", ")
}
