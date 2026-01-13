use spec_trait_macro::{spec, when};

type MyAlias = u8;
type MyVecAlias = Vec<i32>;

struct BaseType;

trait TypesTrait<T> {
    fn types_method(&self, _x: T);
}

impl<T> TypesTrait<T> for BaseType {
    fn types_method(&self, _x: T) {
        println!("Default");
    }
}

#[when(T = MyAlias)]
impl<T> TypesTrait<T> for BaseType {
    fn types_method(&self, _x: T) {
        println!("T is MyAlias");
    }
}

#[when(T = Vec<MyAlias>)]
impl<T> TypesTrait<T> for BaseType {
    fn types_method(&self, _x: T) {
        println!("T is Vec<u8>");
    }
}

#[when(T = Vec<_>)]
impl<T> TypesTrait<T> for BaseType {
    fn types_method(&self, _x: T) {
        println!("T is Vec<_>");
    }
}

#[when(T = MyVecAlias)]
impl<T> TypesTrait<T> for BaseType {
    fn types_method(&self, _x: T) {
        println!("T is MyVecAlias");
    }
}

#[when(T = (i32, _))]
impl<T> TypesTrait<T> for BaseType {
    fn types_method(&self, _x: T) {
        println!("T is (i32, _)");
    }
}

#[when(T = &[i32])]
impl<T> TypesTrait<T> for BaseType {
    fn types_method(&self, _x: T) {
        println!("T is &[i32]");
    }
}

#[when(T = fn(&u8) -> &u8)]
impl<T> TypesTrait<T> for BaseType {
    fn types_method(&self, _x: T) {
        println!("T is a function pointer from &u8 to &u8");
    }
}

pub fn run() {
    println!("\n- Types Examples:");

    let x = BaseType;

    spec! { x.types_method(1u8); BaseType; [MyAlias] } // -> "T is MyAlias"
    spec! { x.types_method(1u8); BaseType; [u8]; u8 = MyAlias } // -> "T is MyAlias"
    spec! { x.types_method(vec![1i32]); BaseType; [Vec<i32>]; Vec<i32> = MyVecAlias } // -> "T is MyVecAlias"
    spec! { x.types_method(vec![1u8]); BaseType; [Vec<u8>]; u8 = MyAlias } // -> "T is Vec<u8>"
    spec! { x.types_method(vec![1i32]); BaseType; [Vec<i32>] } // -> "T is Vec<_>"
    spec! { x.types_method((1, 2)); BaseType; [(i32, i32)] } // -> "T is (i32, _)"
    spec! { x.types_method(&[1i32]); BaseType; [&[i32]] } // -> "T is &[i32]"
    spec! { x.types_method(|x: &u8| x); BaseType; [fn(&u8) -> &u8] } // -> "T is a function pointer from &u8 to &u8"
    spec! { x.types_method(1i8); BaseType; [i8] } // -> "Default"
}
