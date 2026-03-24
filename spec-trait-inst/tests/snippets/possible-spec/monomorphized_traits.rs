struct S;

trait FooI32 {
    fn foo(&self, x: i32);
}

trait FooStr {
    fn foo(&self, x: &str);
}

impl FooI32 for S {
    fn foo(&self, x: i32) {
        println!("Called FooI32::foo with {}", x);
    }
}

impl FooStr for S {
    fn foo(&self, x: &str) {
        println!("Called FooStr::foo with {}", x);
    }
}
