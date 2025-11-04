struct S;

impl S {
    fn foo_i32(&self, x: i32) {
        println!("Called foo_i32 with {}", x);
    }

    fn foo_str(&self, x: &str) {
        println!("Called foo_str with {}", x);
    }
}
