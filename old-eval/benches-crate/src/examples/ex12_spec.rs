// Example 12 — WITH macros: Fill / memset specialization
use spec_trait_macro::{spec, when};

pub struct Filler;

pub trait FillTrait<T> {
    fn fill(&self, buf: &mut [T], val: T);
}

impl<T: Clone> FillTrait<T> for Filler {
    #[inline(never)]
    fn fill(&self, buf: &mut [T], val: T) {
        for slot in buf.iter_mut() {
            *slot = val.clone();
        }
    }
}

#[when(T = u8)]
impl<T> FillTrait<T> for Filler {
    #[inline(never)]
    fn fill(&self, buf: &mut [T], val: T) {
        // memset fast path
        unsafe {
            std::ptr::write_bytes(buf.as_mut_ptr(), val, buf.len());
        }
    }
}

#[when(T = i32)]
impl<T> FillTrait<T> for Filler {
    #[inline(never)]
    fn fill(&self, buf: &mut [T], val: T) {
        buf.fill(val);
    }
}

pub fn fill_u8(f: &Filler, buf: &mut [u8], val: u8) {
    spec! { f.fill(buf, val); Filler; [&mut [u8], u8] }
}

pub fn fill_i32(f: &Filler, buf: &mut [i32], val: i32) {
    spec! { f.fill(buf, val); Filler; [&mut [i32], i32] }
}

pub fn fill_generic<T: Clone>(f: &Filler, buf: &mut [T], val: T) {
    FillTrait::fill(f, buf, val)
}
