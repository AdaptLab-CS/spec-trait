// src/lib.rs
extern crate spec_trait_macro;

use spec_trait_macro::{specializable, when};

struct ZST; // Zero Sized Type

#[specializable]
trait Foo<T> {
    fn foo(&self, x: T);
}

#[specializable]
trait Bar {
    fn bar(&self);
}

#[when(not(all(T = TypeName, any(T: TraitName, U: TraitName, X = &String), not(U: TraitName1 + TraitName2))))]
impl<T> Foo<T> for ZST {
    fn foo(&self, x: T) {
        println!("Foo for ZST");
    }
}

#[when(not(T: TraitName))]
impl<T: 'static> Bar for T {
    fn bar(&self) {
        println!("Bar for T");
    }
}

fn main() {
    println!("Hello, world! (from spec-trait-bin)");
}
