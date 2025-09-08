use spec_trait_macro::{ spec, when };

struct ZST;
struct ZST2;

trait Foo<T> { fn foo(&self, x: T); }

trait Foo2<T, U> {
    fn foo(&self, x: T, y: U);
}

trait Foo3<T> { fn foo(&self, x: T, y: String); }

type MyType = u8;
type MyVecAlias = Vec<i32>;

trait Bar {}
trait FooBar {}

impl Bar for i32 {}
impl Bar for i64 {}
impl FooBar for i64 {}

// ZST - Foo

impl<T> Foo<T> for ZST {
    fn foo(&self, x: T) {
        println!("Default Foo for ZST");
    }
}

#[when(T = MyType)]
impl<T> Foo<T> for ZST {
    fn foo(&self, x: T) {
        println!("Foo impl ZST where T is MyType");
    }
}

#[when(T: Bar)]
impl<T> Foo<T> for ZST {
    fn foo(&self, x: T) {
        println!("Foo impl ZST where T implements Bar");
    }
}

#[when(T: Bar + FooBar)]
impl<T> Foo<T> for ZST {
    fn foo(&self, x: T) {
        println!("Foo impl ZST where T implements Bar and FooBar");
    }
}

#[when(T = Vec<MyType>)]
impl<T> Foo<T> for ZST {
    fn foo(&self, x: T) {
        println!("Foo impl ZST where T is Vec<u8>");
    }
}

#[when(T = Vec<_>)]
impl<T> Foo<T> for ZST {
    fn foo(&self, x: T) {
        println!("Foo impl ZST where T is Vec<_>");
    }
}

#[when(T = MyVecAlias)]
impl<T> Foo<T> for ZST {
    fn foo(&self, x: T) {
        println!("Foo impl ZST where T is MyVecAlias");
    }
}

#[when(T = (i32, _))]
impl<T> Foo<T> for ZST {
    fn foo(&self, x: T) {
        println!("Foo impl ZST where T is (i32, _)");
    }
}


#[when(T = &[i32])]
impl<T> Foo<T> for ZST {
    fn foo(&self, x: T) {
        println!("Foo impl ZST where T is &[i32]");
    }
}

// ZST - Foo2

impl<T, U> Foo2<T, U> for ZST {
    fn foo(&self, x: T, y: U) {
        println!("Default Foo2 for ZST");
    }
}

#[when(T = MyType)]
impl<T, U> Foo2<T, U> for ZST {
    fn foo(&self, x: T, y: U) {
        println!("Foo2 for ZST where T is MyType");
    }
}

// ZST - Foo3
#[when(T = String)]
impl<T> Foo3<T> for ZST {
    fn foo(&self, x: T, y: String) {
        println!("Foo3 for ZST where T is String");
    }
}


// ZST2 - Foo

impl<T> Foo<T> for ZST2 {
    fn foo(&self, x: T) {
        println!("Default Foo for ZST2");
    }
}

#[when(T = MyType)]
impl<T> Foo<T> for ZST2 {
    fn foo(&self, x: T) {
        println!("Foo impl ZST2 where T is MyType");
    }
}

#[when(all(T = Vec<i32>, T = Vec<_>))]
impl<T> Foo<T> for ZST2 {
    fn foo(&self, x: T) {
        println!("Foo impl ZST2 where T is Vec<i32>");
    }
}

#[when(any(T: Copy, T: Clone))]
impl<T> Foo<T> for ZST2 {
    fn foo(&self, x: T) {
        println!("Foo impl ZST2 where T implements Copy or Clone");
    }
}

// ZST2 - Foo2

impl<T, U> Foo2<T, U> for ZST2 where T: 'static {
    fn foo(&self, x: T, y: U) {
        println!("Default Foo2 for ZST2");
    }
}

#[when(T = MyType)]
impl<T: 'static, U> Foo2<T, U> for ZST2 {
    fn foo(&self, x: T, y: U) {
        println!("Foo2 for ZST2 where T is MyType");
    }
}

#[when(not(T = MyType))]
impl<T: 'static, U> Foo2<T, U> for ZST2 where T: 'static {
    fn foo(&self, x: T, y: U) {
        println!("Foo2 for ZST2 where T is not MyType");
    }
}

fn main() {
    let zst = ZST;
    let zst2 = ZST2;

    // ZST - Foo
    spec! { zst.foo(1u8); ZST; [u8]; u8 = MyType }
    spec! { zst.foo(vec![1i32]); ZST; [Vec<i32>]; Vec<i32> = MyVecAlias }
    spec! { zst.foo(vec![1u8]); ZST; [Vec<u8>]; u8 = MyType }
    spec! { zst.foo(vec![1i32]); ZST; [Vec<i32>] }
    spec! { zst.foo((1, 2)); ZST; [(i32, i32)] }
    spec! { zst.foo(&[1i32]); ZST; [&[i32]] }
    spec! { zst.foo(1i32); ZST; [i32]; i32: Bar  }
    spec! { zst.foo(1i64); ZST; [i64]; i64: Bar + FooBar }
    spec! { zst.foo(1i8); ZST; [i8] }

    // ZST - Foo2
    spec! { zst.foo(1u8, 2u8); ZST; [u8, u8]; u8 = MyType }
    spec! { zst.foo(1i32, 1i32); ZST; [i32, i32] }


    // ZST - Foo3
    spec! { zst.foo("hello".to_string(), "world".to_string()); ZST; [String, String] }

    // ZST2 - Foo
    spec! { zst2.foo(1u8); ZST2; [u8]; u8 = MyType }
    spec! { zst2.foo(vec![1i32]); ZST2; [Vec<i32>] } 
    spec! { zst2.foo(1i32); ZST2; [i32]; i32: Copy  }
    spec! { zst2.foo(1i32); ZST2; [i32] }

    // ZST2 - Foo2
    spec! { zst2.foo(1u8, 2u8); ZST2; [u8, u8]; u8 = MyType }
    spec! { zst2.foo(1i8, 1i8); ZST2; [i8, i8] }
}
