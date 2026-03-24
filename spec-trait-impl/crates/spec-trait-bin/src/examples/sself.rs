use spec_trait_macro::{spec, when};

struct BaseType1;
struct BaseType2;

trait SelfTrait<T> {
    fn self_method(&self, _x: T);
}

impl<T> SelfTrait<T> for BaseType1 {
    fn self_method(&self, _x: T) {
        println!("BaseType1: Default");
    }
}

impl<T> SelfTrait<T> for BaseType2 {
    fn self_method(&self, _x: T) {
        println!("BaseType2: Default");
    }
}

impl<T, U> SelfTrait<T> for Vec<U> {
    fn self_method(&self, _x: T) {
        println!("Vec<U>: Default");
    }
}

impl<T, U, V> SelfTrait<T> for (U, V) {
    fn self_method(&self, _x: T) {
        println!("(U, V): Default");
    }
}

impl<T> SelfTrait<T> for i32 {
    fn self_method(&self, _x: T) {
        println!("i32: Default");
    }
}

#[when(T = u8)]
impl<T> SelfTrait<T> for BaseType1 {
    fn self_method(&self, _x: T) {
        println!("BaseType1: T is u8");
    }
}

#[when(T = u8)]
impl<T> SelfTrait<T> for BaseType2 {
    fn self_method(&self, _x: T) {
        println!("BaseType2: T is u8");
    }
}

#[when(T = u8)]
impl<T, U> SelfTrait<T> for Vec<U> {
    fn self_method(&self, _x: T) {
        println!("Vec<U>: T is u8");
    }
}

#[when(T = u8)]
impl<T, U, V> SelfTrait<T> for (U, V) {
    fn self_method(&self, _x: T) {
        println!("(U, V): T is u8");
    }
}

#[when(T = u8)]
impl<T> SelfTrait<T> for i32 {
    fn self_method(&self, _x: T) {
        println!("i32: T is u8");
    }
}

pub fn run() {
    println!("\n- Self Type Examples:");

    let x = BaseType1;
    spec! { x.self_method(42u8); BaseType1; [u8] } // -> "BaseType1: T is u8"
    spec! { x.self_method(1i32); BaseType1; [i32] } // -> "BaseType1: Default"

    let y = BaseType2;
    spec! { y.self_method(42u8); BaseType2; [u8] } // -> "BaseType2: T is u8"
    spec! { y.self_method(1i32); BaseType2; [i32] } // -> "BaseType2: Default"

    spec! { vec![1u8].self_method(42u8); Vec<u8>; [u8] } // -> "Vec<U>: T is u8"
    spec! { vec![1i32].self_method(1i32); Vec<i32>; [i32] } // -> "Vec<U>: Default"

    spec! { (1u8, 2u8).self_method(42u8); (u8, u8); [u8] } // -> "(U, V): T is u8"
    spec! { (1i32, 2i32).self_method(1i32); (i32, i32); [i32] } // -> "(U, V): Default"

    spec! { 1i32.abs().self_method(42u8); i32; [u8] } // -> "i32: T is u8" 
    spec! { 1i32.abs().self_method(1i32); i32; [i32] } // -> "i32: Default" 
}
