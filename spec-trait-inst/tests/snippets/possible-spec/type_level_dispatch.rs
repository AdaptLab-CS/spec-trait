struct S;
struct SWithV<T> {
    value: T,
}

trait Foo {
    fn foo(&self);
}

impl Foo for SWithV<i32> {
    fn foo(&self) {
        println!("Called Foo::foo with {}", self.value);
    }
}

impl Foo for SWithV<&str> {
    fn foo(&self) {
        println!("Called Foo::foo with {}", self.value);
    }
}
