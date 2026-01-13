use spec_trait_macro::{spec, when};

struct BaseType;

trait BaseTrait<T> {
    fn base_method(&self, _x: T);
}

impl<T> BaseTrait<T> for BaseType {
    fn base_method(&self, _x: T) {
        println!("Default");
    }
}

#[when(T = u8)]
impl<T> BaseTrait<T> for BaseType {
    fn base_method(&self, _x: T) {
        println!("T is u8");
    }
}

#[when(T: Copy)]
impl<T> BaseTrait<T> for BaseType {
    fn base_method(&self, _x: T) {
        println!("T implements Copy");
    }
}

#[when(any(T = String, T = i32))]
impl<T> BaseTrait<T> for BaseType {
    fn base_method(&self, _x: T) {
        println!("T is String or i32");
    }
}

#[when(all(T = Vec<U>, U: Copy))]
impl<T, U> BaseTrait<T> for BaseType {
    fn base_method(&self, _x: T) {
        println!("T is a Vec of Copy");
    }
}

#[when(not(T = &BaseType))]
impl<T> BaseTrait<T> for BaseType {
    fn base_method(&self, _x: T) {
        println!("T is not &BaseType");
    }
}

pub fn run() {
    println!("\n- Base Examples:");

    let x = BaseType;

    spec! { x.base_method(42u8); BaseType; [u8] } // T is u8
    spec! { x.base_method(1.2f32); BaseType; [f32]; f32: Copy } // T implements Copy
    spec! { x.base_method(1i32); BaseType; [i32] } // T is String or i32
    spec! { x.base_method(vec![1, 2, 3]); BaseType; [Vec<i32>]; i32: Copy } // T is a Vec of Copy
    spec! { x.base_method(1i8); BaseType; [i8] } // T is not &BaseType
    spec! { x.base_method(&x); BaseType; [&BaseType] } // Default
}
