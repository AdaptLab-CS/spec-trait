use spec_trait_macro::{spec, when};

struct BaseType;

#[allow(dead_code)]
trait HigherOrderSpecTrait<F, T> {
    fn higher_order_spec_method(&self, f: F, arg: T) -> T;
}

#[when(F = for<'a> fn(&'a T) -> T)]
impl<F, T> HigherOrderSpecTrait<F, T> for BaseType {
    fn higher_order_spec_method(&self, f: F, arg: T) -> T {
        println!("T is generic");
        f(&arg)
    }
}

#[when(all(T = i32, F = for<'a> fn(&'a T) -> T))]
impl<F, T> HigherOrderSpecTrait<F, T> for BaseType {
    fn higher_order_spec_method(&self, f: F, arg: T) -> T {
        println!("T is i32");
        f(&arg)
    }
}

pub fn run() {
    println!("\n- Higher Order Specialization Examples:");

    let x = BaseType;

    let _: i32 = spec! { x.higher_order_spec_method(|v: &i32| *v, 1i32); BaseType; [for<'a> fn(&'a i32) -> i32, i32] }; // -> "T is i32"
    let _: u32 = spec! { x.higher_order_spec_method(|v: &u32| *v, 1u32); BaseType; [for<'a> fn(&'a u32) -> u32, u32] }; // -> "T is generic"
}
