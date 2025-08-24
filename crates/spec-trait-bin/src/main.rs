use spec_trait_macro::{ spec, spec_default, when };

struct ZST;
struct ZST2;

trait Foo<T> { fn foo(&self, x: T); }

trait Foo2<T, U> {
    fn foo(&self, x: T, y: U);
}

type MyType = u8;

trait Bar {}
trait FooBar {}

impl Bar for i32 {}
impl Bar for i64 {}
impl FooBar for i64 {}

// ZST - Foo

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

// ZST - Foo2

#[spec_default]
impl<T, U> Foo2<T, U> for ZST {
    fn foo(&self, x: T, y: U) {
        println!("Default Foo2 for ZST");
    }
}

#[when(T = MyType)]
impl<T, U> Foo2<T, U> for ZST {
    fn foo(&self, x: T, y: U) {
        println!("Foo2 for ZST where T is MyType");
    }
}


// ZST2 - Foo

#[spec_default]
impl<T> Foo<T> for ZST2 {
    fn foo(&self, x: T) {
        println!("Default Foo for ZST2");
    }
}

#[when(T = MyType)]
impl<T> Foo<T> for ZST2 {
    fn foo(&self, x: T) {
        println!("Foo impl ZST2 where T is MyType");
    }
}

fn main() {
    println!("Hello, world! (from spec-trait-bin)");
    let zst = ZST;
    let zst2 = ZST2;

    // ZST - Foo
    spec! { zst.foo(1u8); ZST; [u8]; u8 = MyType }
    spec! { zst.foo(1i32); ZST; [i32]; i32: Bar  }
    spec! { zst.foo(1i64); ZST; [i64]; i64: Bar + FooBar }
    spec! { zst.foo(1i8); ZST; [i8] }

    // ZST - Foo2
    spec! { zst.foo(1u8, 2u8); ZST; [u8, u8]; u8 = MyType }
    spec! { zst.foo(1i32, 1i32); ZST; [i32, i32] }

    // ZST2 - Foo
    spec! { zst2.foo(1u8); ZST2; [u8]; u8 = MyType }
    spec! { zst2.foo(1i8); ZST2; [i8] }
}
