// Example 6 — WITH macros: ToString specialization
use spec_trait_macro::{spec, when};

pub struct Stringer;

pub trait ToStrTrait<T> {
    fn to_str(&self, val: &T) -> String;
}

impl<T: std::fmt::Debug> ToStrTrait<T> for Stringer {
    #[inline(never)]
    fn to_str(&self, val: &T) -> String {
        format!("{:?}", val)
    }
}

#[when(T = i32)]
impl<T> ToStrTrait<T> for Stringer {
    #[inline(never)]
    fn to_str(&self, val: &T) -> String {
        itoa::Buffer::new().format(*val).to_owned()
    }
}

#[when(T = u64)]
impl<T> ToStrTrait<T> for Stringer {
    #[inline(never)]
    fn to_str(&self, val: &T) -> String {
        itoa::Buffer::new().format(*val).to_owned()
    }
}

#[when(T = f64)]
impl<T> ToStrTrait<T> for Stringer {
    #[inline(never)]
    fn to_str(&self, val: &T) -> String {
        ryu::Buffer::new().format(*val).to_owned()
    }
}

#[when(T = String)]
impl<T> ToStrTrait<T> for Stringer {
    #[inline(never)]
    fn to_str(&self, val: &T) -> String {
        val.clone()
    }
}

pub fn tostr_i32(s: &Stringer, val: &i32) -> String {
    spec! { s.to_str(val); Stringer; [i32] }
}

pub fn tostr_f64(s: &Stringer, val: &f64) -> String {
    spec! { s.to_str(val); Stringer; [f64] }
}

pub fn tostr_string(s: &Stringer, val: &String) -> String {
    spec! { s.to_str(val); Stringer; [String] }
}

pub fn tostr_generic<T: std::fmt::Debug>(s: &Stringer, val: &T) -> String {
    ToStrTrait::to_str(s, val)
}
