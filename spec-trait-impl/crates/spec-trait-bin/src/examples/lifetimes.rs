use spec_trait_macro::{spec, when};

struct BaseType;

trait LifetimesTrait<T> {
    fn lifetimes_method(&self, _x: T);
}

impl<T> LifetimesTrait<T> for BaseType {
    fn lifetimes_method(&self, _x: T) {
        println!("Default");
    }
}

#[when(all(T = &_, T: 'a))]
impl<'a, T> LifetimesTrait<T> for BaseType {
    fn lifetimes_method(&self, _x: T) {
        println!("T is &'a _");
    }
}

#[when(T = &'static _)]
impl<T> LifetimesTrait<T> for BaseType {
    fn lifetimes_method(&self, _x: T) {
        println!("T is &'static _");
    }
}

pub fn run() {
    println!("\n- Lifetimes Examples:");

    let x = BaseType;

    spec! { x.lifetimes_method(&1i32); BaseType; [&'static i32] } // -> "T is &'static _"
    spec! { x.lifetimes_method(&1i32); BaseType; [&i32]; &i32: 'static } // -> "T is &'static _"
    spec! { x.lifetimes_method(&1i32); BaseType; [&'a i32] } // -> "T is &'a _"
    spec! { x.lifetimes_method(&1i32); BaseType; [&i32]; &i32: 'a } // -> "T is &'a _"
    spec! { x.lifetimes_method(&1i32); BaseType; [&i32] } // -> "T is &'a _"
    spec! { x.lifetimes_method(1i32); BaseType; [i32] } // -> "Default"
}
