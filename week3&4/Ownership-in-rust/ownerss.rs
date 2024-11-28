///Example: Ownership and Transfer of Ownership
fn main() {
    let s1 = String::from("Hello"); // s1 owns the String
    let s2 = s1; // Ownership of the String is moved from s1 to s2
    
    // println!("{}", s1); // This would give an error because s1 no longer owns the string
    
    println!("{}", s2); // This works because s2 is now the owner
}
