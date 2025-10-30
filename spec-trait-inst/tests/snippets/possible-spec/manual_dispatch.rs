struct S;

trait Foo {
    fn foo_i32(&self, x: i32);
    fn foo_str(&self, x: &str);
}

impl Foo for S {
    fn foo_i32(&self, x: i32) {
        println!("Called Foo::foo_i32 with {}", x);
    }

    fn foo_str(&self, x: &str) {
        println!("Called Foo::foo_str with {}", x);
    }
}
