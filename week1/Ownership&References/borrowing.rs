fn main() {
    let s = String::from("Hello");
    let len = calculate_length(&s); // Pass by reference
    println!("The length of '{}' is {}", s, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
