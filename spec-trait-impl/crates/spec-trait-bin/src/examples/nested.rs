use spec_trait_macro::{spec, when};

struct BaseType;

trait NestedTrait<T, U> {
    fn nested_method(&self, _x: T, _y: U);
}

impl<T, U> NestedTrait<T, U> for BaseType {
    fn nested_method(&self, _x: T, _y: U) {
        println!("Default");
    }
}

#[when(all(U = Vec<T>, T = u8))]
impl<T, U> NestedTrait<T, U> for BaseType {
    fn nested_method(&self, _x: T, _y: U) {
        println!("T is u8 and U is Vec<u8>");
    }
}

#[when(U = Vec<T>)]
impl<T, U> NestedTrait<T, U> for BaseType {
    fn nested_method(&self, _x: T, _y: U) {
        println!("U is Vec<T>");
    }
}

pub fn run() {
    println!("\n- Nested Examples:");

    let x = BaseType;

    spec! { x.nested_method(1u8, vec![2u8]); BaseType; [u8, Vec<u8>] } // "T is u8 and U is Vec<u8>"
    spec! { x.nested_method(1i32, vec![2i32]); BaseType; [i32, Vec<i32>] } // "U is Vec<T>"
    spec! { x.nested_method(1i32, 2i32); BaseType; [i32, i32] } // -> "Default"
}
