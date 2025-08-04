// src/lib.rs
extern crate spec_trait_macro;

use spec_trait_macro::{spec, spec_default, specializable, when};

struct ZST; // Zero Sized Type

#[specializable]
trait Foo<T> {
    fn foo(&self, x: T);
}

#[specializable]
trait Bar {
    fn bar(&self);
}

#[specializable]
trait FooWithMultipleFns<T> {
    fn foo1(&self, x: T);
    fn foo2(&self, x: T);
}

type MyString = String;

#[spec_default]
impl<T> Foo<T> for ZST {
    fn foo(&self, x: T) {
        println!("Default Foo for ZST");
    }
}

#[when(not(all(T = TypeName, any(T: TraitName, U: TraitName, X = &String), not(U: TraitName1 + TraitName2))))]
impl<T> Foo<T> for ZST {
    fn foo(&self, x: T) {
        println!("Foo for ZST");
    }
}

#[when(all(any(T = String, T = i32), any(T: TraitName1, T: TraitName2)))]
impl<T: 'static> Bar for T {
    fn bar(&self) {
        println!("Bar for T");
    }
}

#[when(any(all(T = String, T = i32), all(T: TraitName1, T: TraitName2)))]
impl<T: 'static> Bar for T {
    fn bar(&self) {
        println!("Bar for T");
    }
}

#[when(not(T: TraitName))]
impl<T: 'static> Bar for T {
    fn bar(&self) {
        println!("Bar for T");
    }
}

#[when(T = MyString)]
impl<T> Foo<T> for ZST {
    fn foo(&self, x: T) {
        println!("Foo<MyString> for ZST");
    }
}

#[when(not(T: TraitName))]
impl<T: 'static> Bar for T {
    fn bar(&self) {
        println!("Bar for T");
    }
}

#[when(T: TraitName2)]
impl<T> FooWithMultipleFns<T> for ZST {
    fn foo1(&self, x: T) {
        println!("FooWithMultipleFns<TraitName2>::foo1 for ZST");
    }

    fn foo2(&self, x: T) {
        println!("FooWithMultipleFns<TraitName2>::foo2 for ZST");
    }
}

fn main() {
    println!("Hello, world! (from spec-trait-bin)");
    spec! { zst.foo(1u8) }
    spec! { zst.foo(1i32); i32: Foo<i32> + Bar; String = MyString; &i32: Bar, &String = &MyString }
    spec! {
        zst.foo(1u8);
        i32: Foo<i32> + Bar;
    }
}
