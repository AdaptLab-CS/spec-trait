// Example 5 — WITHOUT macros: Serialization
use std::any::TypeId;

// --- dyn dispatch ---
pub trait DynSerialize {
    fn serialize_dyn(&self) -> Vec<u8>;
}
impl DynSerialize for i32 {
    #[inline(never)]
    fn serialize_dyn(&self) -> Vec<u8> { self.to_le_bytes().to_vec() }
}
impl DynSerialize for f64 {
    #[inline(never)]
    fn serialize_dyn(&self) -> Vec<u8> { self.to_le_bytes().to_vec() }
}
impl DynSerialize for String {
    #[inline(never)]
    fn serialize_dyn(&self) -> Vec<u8> {
        let bytes = self.as_bytes();
        let mut out = Vec::with_capacity(4 + bytes.len());
        out.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
        out.extend_from_slice(bytes);
        out
    }
}

pub fn ser_dyn(val: &dyn DynSerialize) -> Vec<u8> {
    val.serialize_dyn()
}

// --- enum dispatch ---
pub enum SerEnum<'a> {
    I32(i32),
    F64(f64),
    Str(&'a str),
}
#[inline(never)]
pub fn ser_enum_serialize(variant: &SerEnum<'_>) -> Vec<u8> {
    match variant {
        SerEnum::I32(v) => v.to_le_bytes().to_vec(),
        SerEnum::F64(v) => v.to_le_bytes().to_vec(),
        SerEnum::Str(s) => {
            let bytes = s.as_bytes();
            let mut out = Vec::with_capacity(4 + bytes.len());
            out.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
            out.extend_from_slice(bytes);
            out
        }
    }
}

// --- runtime TypeId ---
#[inline(never)]
pub fn ser_runtime<T: std::fmt::Debug + 'static>(val: &T) -> Vec<u8> {
    let tid = TypeId::of::<T>();
    if tid == TypeId::of::<i32>() {
        let v = unsafe { &*(val as *const T as *const i32) };
        v.to_le_bytes().to_vec()
    } else if tid == TypeId::of::<f64>() {
        let v = unsafe { &*(val as *const T as *const f64) };
        v.to_le_bytes().to_vec()
    } else if tid == TypeId::of::<String>() {
        let v = unsafe { &*(val as *const T as *const String) };
        let bytes = v.as_bytes();
        let mut out = Vec::with_capacity(4 + bytes.len());
        out.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
        out.extend_from_slice(bytes);
        out
    } else {
        format!("{:?}", val).into_bytes()
    }
}

// --- generic fallback ---
#[inline(never)]
pub fn ser_generic<T: std::fmt::Debug>(val: &T) -> Vec<u8> {
    format!("{:?}", val).into_bytes()
}
