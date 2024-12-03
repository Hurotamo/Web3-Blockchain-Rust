fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

fn main() {
    let s1 = String::from("apple");
    let s2 = String::from("banana");

    let result = longest(&s1, &s2);
    println!("The longest string is: {}", result);
}

//In this example, 'a is a lifetime parameter that ensures both references s1 and s2 are valid for the same duration, and the result will be valid as long as both s1 and s2 are valid.
