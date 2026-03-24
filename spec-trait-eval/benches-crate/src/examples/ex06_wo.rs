// Example 6 — WITHOUT macros: ToString
use std::any::TypeId;

// --- dyn dispatch ---
pub trait DynToStr {
    fn to_str_dyn(&self) -> String;
}
impl DynToStr for i32 {
    #[inline(never)]
    fn to_str_dyn(&self) -> String { itoa::Buffer::new().format(*self).to_owned() }
}
impl DynToStr for f64 {
    #[inline(never)]
    fn to_str_dyn(&self) -> String { ryu::Buffer::new().format(*self).to_owned() }
}
impl DynToStr for String {
    #[inline(never)]
    fn to_str_dyn(&self) -> String { self.clone() }
}

pub fn tostr_dyn(val: &dyn DynToStr) -> String {
    val.to_str_dyn()
}

// --- enum dispatch ---
pub enum ToStrEnum<'a> {
    I32(i32),
    F64(f64),
    Str(&'a String),
}
#[inline(never)]
pub fn tostr_enum(variant: &ToStrEnum<'_>) -> String {
    match variant {
        ToStrEnum::I32(v) => itoa::Buffer::new().format(*v).to_owned(),
        ToStrEnum::F64(v) => ryu::Buffer::new().format(*v).to_owned(),
        ToStrEnum::Str(s) => (*s).clone(),
    }
}

// --- runtime TypeId ---
#[inline(never)]
pub fn tostr_runtime<T: std::fmt::Debug + 'static>(val: &T) -> String {
    let tid = TypeId::of::<T>();
    if tid == TypeId::of::<i32>() {
        let v = unsafe { &*(val as *const T as *const i32) };
        itoa::Buffer::new().format(*v).to_owned()
    } else if tid == TypeId::of::<f64>() {
        let v = unsafe { &*(val as *const T as *const f64) };
        ryu::Buffer::new().format(*v).to_owned()
    } else if tid == TypeId::of::<String>() {
        let v = unsafe { &*(val as *const T as *const String) };
        v.clone()
    } else {
        format!("{:?}", val)
    }
}

// --- generic fallback ---
#[inline(never)]
pub fn tostr_generic<T: std::fmt::Debug>(val: &T) -> String {
    format!("{:?}", val)
}
