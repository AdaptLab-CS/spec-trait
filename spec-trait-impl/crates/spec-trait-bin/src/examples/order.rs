use spec_trait_macro::{spec, when};

pub fn run() {
    println!("\n- Order Examples:");

    let x = BaseType;

    spec! { x.order_method(1u8); BaseType; [u8]  } // -> "T is u8"
    spec! { x.order_method(1i8); BaseType; [i8] } // -> "Default"
}

struct BaseType;

trait OrderTrait<T> {
    fn order_method(&self, _x: T);
}

impl<T> OrderTrait<T> for BaseType {
    fn order_method(&self, _x: T) {
        println!("Default");
    }
}

#[when(T = u8)]
impl<T> OrderTrait<T> for BaseType {
    fn order_method(&self, _x: T) {
        println!("T is u8");
    }
}
