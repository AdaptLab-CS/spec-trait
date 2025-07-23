// src/lib.rs
extern crate spec_trait_macro;

use spec_trait_macro::when;

struct ZST; // Zero Sized Type

trait Foo<T> {
    fn foo(&self, x: T);
}

#[when(not(all(T = TypeName, any(T: TraitName, U: TraitName, X = &String), not(U: TraitName1 + TraitName2))))]
impl<T> Foo<T> for ZST {
    fn foo(&self, x: T) {
        println!("Default Foo for ZST");
    }
}

fn main() {
    println!("Hello, world! (from spec-trait-bin)");
}
