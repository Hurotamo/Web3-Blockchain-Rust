fn main() {
    let s = String::from("Hello");
    takes_ownership(s); // s is moved
    // println!("{}", s); // This would cause a compile error

    let x = 5;
    makes_copy(x); // x is copied
    println!("{}", x);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
