use spec_trait_macro::{spec, when};

trait Bad11 {
    fn bad11(&self);
}

impl<T> Bad11 for T {
    fn bad11(&self) {
        println!("generic");
    }
}

#[when(T: 'static)]
impl<T> Bad11 for T {
    fn bad11(&self) {
        println!("'static");
    }
}

pub fn run() {
    println!("\n- Lifetimes 3 Examples:");

    let static_str: &'static str = "hello";
    spec! { "hello".bad11(); &'static str }
    spec! { "hello".bad11(); &'static str; &'static str: 'static }
    spec! { "hello".bad11(); &str; &str: 'static }
    spec! { static_str.bad11(); &'static str }
    spec! { static_str.bad11(); &'static str; &'static str: 'static }
    spec! { static_str.bad11(); &str; &str: 'static }
    <&'static str as Bad11>::bad11(&static_str);
}
