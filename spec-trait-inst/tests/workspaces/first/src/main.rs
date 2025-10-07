#[derive(Clone)]
struct T{
    value: i32,
}

fn test_own(t: T) {
    // Set the value to 0
    let mut t = t;
    t.value = 0;
}

fn test_bor(t: &T) {
    let _ = t;
}

fn test_mut_bor(t: &mut T) {
    let _ = t;
}

fn main() {
    let t1 = T { value: 10 };
    let mut t2 = T { value: 10 };

    test_own(t1.clone()); // This is a `move` where the value is not in the `local_var_decl`
    test_bor(&t1);
    test_mut_bor(&mut t2);
    test_own(t1);
}
