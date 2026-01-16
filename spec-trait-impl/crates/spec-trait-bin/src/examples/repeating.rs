use spec_trait_macro::{spec, when};

struct BaseType;

trait RepeatingGenericsTrait<T, U> {
    fn repeating_generics_method(&self, _x: T, _y: U);
}

impl<T, U> RepeatingGenericsTrait<T, U> for BaseType {
    fn repeating_generics_method(&self, _x: T, _y: U) {
        println!("Default");
    }
}

#[when(all(U = Vec<T>, T = u8))]
impl<T, U> RepeatingGenericsTrait<T, U> for BaseType {
    fn repeating_generics_method(&self, _x: T, _y: U) {
        println!("T is u8 and U is Vec<u8>");
    }
}

#[when(U = Vec<T>)]
impl<T, U> RepeatingGenericsTrait<T, U> for BaseType {
    fn repeating_generics_method(&self, _x: T, _y: U) {
        println!("U is Vec<T>");
    }
}

#[when(T = Vec<U>)]
impl<T, U> RepeatingGenericsTrait<T, U> for BaseType {
    fn repeating_generics_method(&self, _x: T, _y: U) {
        println!("T is Vec<U>");
    }
}

#[when(T: Copy)]
impl<T> RepeatingGenericsTrait<T, T> for BaseType {
    fn repeating_generics_method(&self, _x: T, _y: T) {
        println!("T implements Copy for both parameters");
    }
}

#[when(all(T = String, T = i32))]
impl<T> RepeatingGenericsTrait<T, T> for BaseType {
    fn repeating_generics_method(&self, _x: T, _y: T) {
        panic!("T cannot be both String and i32");
    }
}

#[when(all(T = &str, T: 'a, U = &'a i32))]
impl<'a, T, U> RepeatingGenericsTrait<T, U> for BaseType {
    fn repeating_generics_method(&self, _x: T, _y: U) {
        println!("T and U have the same lifetime");
    }
}

#[when(all(T = &str, T: 'a, U = &'b i32))]
impl<'a, 'b, T, U> RepeatingGenericsTrait<T, U> for BaseType {
    fn repeating_generics_method(&self, _x: T, _y: U) {
        println!("T and U have different lifetimes");
    }
}

#[when(all(T = &str, T: 'b, U = for<'a> fn(T, &'a i32)))]
impl<'b, T, U> RepeatingGenericsTrait<T, U> for BaseType {
    fn repeating_generics_method(&self, x: T, y: U) {
        y(x, &42);
    }
}

pub fn run() {
    println!("\n- Repeating Generics Examples:");

    let x = BaseType;

    spec! { x.repeating_generics_method(1u8, vec![2u8]); BaseType; [u8, Vec<u8>] } // "T is u8 and U is Vec<u8>"
    spec! { x.repeating_generics_method(1i32, vec![2i32]); BaseType; [i32, Vec<i32>] } // "U is Vec<T>"
    spec! { x.repeating_generics_method(vec![1i32], 2i32); BaseType; [Vec<i32>, i32] } // "T is Vec<U>"
    spec! { x.repeating_generics_method(1i32, vec![2u8]); BaseType; [i32, Vec<u8>] } // "Default"
    spec! { x.repeating_generics_method(vec![1i32], 2u8); BaseType; [Vec<i32>, u8] } // "Default"

    spec! { x.repeating_generics_method(1i32, 2i32); BaseType; [i32, i32]; i32: Copy } // "T implements Copy for both parameters"
    spec! { x.repeating_generics_method(1i32, 2u8); BaseType; [i32, u8]; i32: Copy; u8: Copy } // "Default"
    spec! { x.repeating_generics_method(1i32, 2u8); BaseType; [i32, u8]; i32: Copy } // "Default"

    spec! { x.repeating_generics_method("test", &2i32); BaseType; [&'static str, &'static i32] } // "T and U have the same lifetimes"
    spec! { x.repeating_generics_method("test", &2i32); BaseType; [&'a str, &'a i32] } // "T and U have the same lifetimes"
    spec! { x.repeating_generics_method("test", &2i32); BaseType; [&'static str, &'a i32] } // "T and U have different lifetimes"
    spec! { x.repeating_generics_method("test", &2i32); BaseType; [&'a str, &'b i32] } // "T and U have different lifetimes"

    spec! {
        x.repeating_generics_method(
            "hello",
            |s: &str, n: &i32| {
                println!("Function called with s: {} and n: {}", s, n);
            }
        );
        BaseType;
        [&'static str, for<'a> fn(&'static str, &'a i32)]
    } // "Function called with s: hello and n: 42"
}
