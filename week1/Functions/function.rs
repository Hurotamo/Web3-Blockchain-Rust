fn main() {
    greet("Alice");
    let result = add(5, 3);
    println!("Sum: {}", result);
}

fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn add(x: i32, y: i32) -> i32 {
    x + y // Return statement without `;`
}
