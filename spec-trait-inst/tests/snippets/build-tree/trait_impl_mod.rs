fn main() {
    trait Tr<T> {
        const C: i32;
        fn a(&self, value: T) -> T;
        fn b(&self) -> i32 {
            42
        }
    }

    impl Tr<i32> for i32 {
        const C: i32 = 10;
        fn a(&self, value: i32) -> i32 {
            *self + value
        }
    }

    struct S;

    impl S {
        fn c(&self) {}
    }

    mod inner {
        fn inner_function() {}
    }
}
