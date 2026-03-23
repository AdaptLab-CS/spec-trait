use spec_trait_macro::{spec, when};

trait Bad1 {
    fn bad1(&self);
}

impl<T> Bad1 for T {
    fn bad1(&self) {
        println!("generic");
    }
}

#[when(T: 'static)]
impl<T> Bad1 for T {
    fn bad1(&self) {
        println!("'static");
    }
}

pub fn run() {
    println!("\n- Lifetimes 2 Examples:");

    let static_str: &'static str = &"hello";
    spec! { "hello".bad1(); &'static str }
    spec! { "hello".bad1(); &'static str; &'static str: 'static }
    spec! { "hello".bad1(); &str; &str: 'static }
    spec! { static_str.bad1(); &'static str }
    spec! { static_str.bad1(); &'static str; &'static str: 'static }
    spec! { static_str.bad1(); &str; &str: 'static }
    <&'static str as Bad1>::bad1(&static_str);
}
