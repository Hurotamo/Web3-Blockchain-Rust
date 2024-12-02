

```markdown
# Rust Syntax Cheat Sheet

Welcome to the Rust Syntax Cheat Sheet! This repository is a quick reference for fundamental Rust syntax, designed to help you understand and use Rust effectively. Whether you're a beginner or an experienced Rustacean, this guide is here to assist.

---

## Table of Contents
- [Variable Declarations](#variable-declarations)
- [Control Flow](#control-flow)
- [Functions](#functions)
- [Enums](#enums)
- [Structs](#structs)
- [Traits and Implementations](#traits-and-implementations)
- [Constants and Static Variables](#constants-and-static-variables)
- [Loops](#loops)
- [Pattern Matching](#pattern-matching)
- [Modules and Visibility](#modules-and-visibility)
- [Other Key Syntax](#other-key-syntax)

---

## Variable Declarations
```rust
// Immutable variable
let x = 10;

// Mutable variable
let mut y = 20;
y = 30; // Allowed because of `mut`
```

---

## Control Flow
```rust
let x = 5;

if x > 3 {
    println!("x is greater than 3");
} else {
    println!("x is 3 or less");
}

// Match statement
let color = "red";
match color {
    "red" => println!("You chose red!"),
    "blue" => println!("You chose blue!"),
    _ => println!("Unknown color"),
}
```

---

## Functions
```rust
// Function without a return value
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

// Function with a return value
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    greet("Alice");
    let sum = add(5, 7);
    println!("Sum: {}", sum);
}
```

---

## Enums
```rust
pub enum Color {
    Red,
    Green,
    Blue,
}

pub enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}
```

---

## Structs
```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle { width: 10, height: 20 };
    println!("Area: {}", rect.area());
}
```

---

## Traits and Implementations
```rust
trait Greet {
    fn greet(&self);
}

struct Person {
    name: String,
}

impl Greet for Person {
    fn greet(&self) {
        println!("Hello, {}!", self.name);
    }
}
```

---

## Constants and Static Variables
```rust
const PI: f64 = 3.14159;

static mut COUNTER: i32 = 0; // Unsafe global mutable variable

fn main() {
    println!("Value of PI: {}", PI);

    unsafe {
        COUNTER += 1;
        println!("Counter: {}", COUNTER);
    }
}
```

---

## Loops
```rust
// Infinite loop
loop {
    println!("Running...");
    break; // Exit the loop
}

// While loop
let mut x = 0;
while x < 5 {
    println!("x: {}", x);
    x += 1;
}

// For loop
for i in 0..5 {
    println!("i: {}", i);
}
```

---

## Pattern Matching
```rust
let point = (0, 5);

match point {
    (0, y) => println!("Point lies on the y-axis at {}", y),
    (x, 0) => println!("Point lies on the x-axis at {}", x),
    _ => println!("Point is somewhere else"),
}
```

---

## Modules and Visibility
```rust
mod my_module {
    pub fn say_hello() {
        println!("Hello from the module!");
    }
}

fn main() {
    my_module::say_hello();
}
```

---

## Other Key Syntax
```rust
// Tuples
let tuple = (1, "hello", 4.5);
println!("First value: {}", tuple.0);

// Arrays
let array = [1, 2, 3, 4, 5];
println!("Third value: {}", array[2]);

// Option type
let some_number = Some(5);
let no_number: Option<i32> = None;

if let Some(value) = some_number {
    println!("The value is: {}", value);
}

// Result type
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}

match divide(10, 2) {
    Ok(result) => println!("Result: {}", result),
    Err(e) => println!("Error: {}", e),
}
```

