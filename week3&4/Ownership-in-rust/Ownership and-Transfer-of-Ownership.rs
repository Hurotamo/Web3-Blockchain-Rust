///Example: Ownership and Transfer of Ownership
fn main() {
    let s1 = String::from("Hello"); // s1 owns the String
    let s2 = s1; // Ownership of the String is moved from s1 to s2
    
    // println!("{}", s1); // This would give an error because s1 no longer owns the string
    
    println!("{}", s2); // This works because s2 is now the owner
}
///In this example, the ownership of s1 is transferred to s2 when s1 is assigned to s2. After this, 
///s1 is no longer valid, and trying to access it results in a compile-time error.
