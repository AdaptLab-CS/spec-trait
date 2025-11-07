struct S;

fn foo_i32(sself: &S, x: i32) {
    println!("Called foo_i32 with {}", x);
}

fn foo_str(sself: &S, x: &str) {
    println!("Called foo_str with {}", x);
}
