fn main() {
    let mut s = String::from("Hello");

    append_world(&mut s); // Borrowing `s` mutably
    println!("{}", s); // s is modified in append_world
}

fn append_world(s: &mut String) {
    s.push_str(", world!"); // Modifying the borrowed data
}

//In this example, s is borrowed mutably inside the function append_world and can be modified there.

