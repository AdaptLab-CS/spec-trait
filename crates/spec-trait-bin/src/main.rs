extern crate spec_trait_macro;

use spec_trait_macro::{ spec, spec_default, specializable, when };

struct ZST; // Zero Sized Type

#[specializable]
trait Foo<T> {
    fn foo(&self, x: T);
}

type MyType = u8;

#[specializable]
trait Bar {}
#[specializable]
trait FooBar {}

impl Bar for i32 {}
impl Bar for i64 {}
impl FooBar for i64 {}

#[spec_default]
impl<T> Foo<T> for ZST {
    fn foo(&self, x: T) {
        println!("Default Foo for ZST");
    }
}

#[when(T = MyType)]
impl<T> Foo<T> for ZST {
    fn foo(&self, x: T) {
        println!("Foo impl ZST where T is MyType");
    }
}

#[when(T: Bar)]
impl<T> Foo<T> for ZST {
    fn foo(&self, x: T) {
        println!("Foo impl ZST where T implements Bar");
    }
}

#[when(T: Bar + FooBar)]
impl<T> Foo<T> for ZST {
    fn foo(&self, x: T) {
        println!("Foo impl ZST where T implements Bar and FooBar");
    }
}

fn main() {
    println!("Hello, world! (from spec-trait-bin)");
    let zst = ZST;
    spec! { zst.foo(1u8); ZST; [u8]; u8 = MyType; i32: Bar; i64: Bar + FooBar }
    spec! { zst.foo(1i32); ZST; [i32]; u8 = MyType; i32: Bar; i64: Bar + FooBar }
    spec! { zst.foo(1i64); ZST; [i64]; u8 = MyType; i32: Bar; i64: Bar + FooBar }
    spec! { zst.foo(1i8); ZST; [i8]; u8 = MyType; i32: Bar; i64: Bar + FooBar }
}
