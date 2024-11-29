fn main() {
    let s = String::from("Hello");

    let len = calculate_length(&s); // Borrowing `s` immutably
    println!("The length of '{}' is {}", s, len); // `s` is still valid here
}

fn calculate_length(s: &String) -> usize {
    s.len() // Access the string without taking ownership
}
