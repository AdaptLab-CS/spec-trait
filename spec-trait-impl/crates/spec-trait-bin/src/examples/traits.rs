use spec_trait_macro::{spec, when};

trait Bar {}
trait FooBar {}

impl Bar for i32 {}
impl Bar for i64 {}
impl FooBar for i64 {}

struct BaseType;

trait TraitsTrait<T> {
    fn traits_method(&self, _x: T);
}

impl<T> TraitsTrait<T> for BaseType {
    fn traits_method(&self, _x: T) {
        println!("Default");
    }
}

#[when(T: Bar)]
impl<T> TraitsTrait<T> for BaseType {
    fn traits_method(&self, _x: T) {
        println!("T implements Bar");
    }
}

#[when(T: Bar + FooBar)]
impl<T> TraitsTrait<T> for BaseType {
    fn traits_method(&self, _x: T) {
        println!("T implements Bar and FooBar");
    }
}

#[when(T: Fn(i32) -> i32)]
impl<T> TraitsTrait<T> for BaseType {
    fn traits_method(&self, _x: T) {
        println!("T implements Fn(i32) -> i32");
    }
}

#[when(T: for<'a> Fn(&'a i32) -> i32)]
impl<T> TraitsTrait<T> for BaseType {
    fn traits_method(&self, _x: T) {
        println!("T implements for<'a> Fn(&'a i32) -> i32");
    }
}

pub fn run() {
    println!("\n- Traits Examples:");

    let x = BaseType;

    spec! { x.traits_method(1i32); BaseType; [i32]; i32: Bar  } // -> "T implements Bar"
    spec! { x.traits_method(1i64); BaseType; [i64]; i64: Bar + FooBar } // -> "T implements Bar and FooBar"
    spec! { x.traits_method(|x: i32| x); BaseType; [T]; T: Fn(i32) -> i32 } // -> "T implements Fn(i32) -> i32"
    spec! { x.traits_method(|x: &i32| *x); BaseType; [T]; T: for<'a> Fn(&'a i32) -> i32 } // -> "T implements for<'a> Fn(&'a i32) -> i32"
    spec! { x.traits_method(1i8); BaseType; [i8] } // -> "Default"
}
