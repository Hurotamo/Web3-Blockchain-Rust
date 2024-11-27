---

### **Basics**

#### **Main Function**
```rust
fn main() {
    println!("Hello, Rust!");
}
```

#### **Variables**
- Immutable by default:
```rust
let x = 5;
```
- Mutable:
```rust
let mut y = 10;
y += 1;
```

#### **Data Types**
- Integer: `i8`, `u8`, `i32`, `u32`, `i64`, `usize`, etc.
- Floating-point: `f32`, `f64`
- Boolean: `bool`
- Character: `char`
- Tuple: `(i32, f64, char)`
- Array: `[i32; 5]`

---

### **Control Flow**

#### **Conditionals**
```rust
if x > 5 {
    println!("x is greater than 5");
} else {
    println!("x is 5 or less");
}
```

#### **Loops**
- Infinite loop:
```rust
loop {
    println!("Running forever!");
    break;
}
```
- While:
```rust
while x < 10 {
    x += 1;
}
```
- For:
```rust
for i in 0..5 {
    println!("{}", i);
}
```

---

### **Functions**
```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

---

### **Ownership & References**

#### **Ownership Rules**
1. Each value has a single owner.
2. Values are dropped when the owner goes out of scope.
3. Borrowing (`&T`), Mutable Borrowing (`&mut T`).

#### **Example**
```rust
fn main() {
    let s = String::from("hello");
    takes_ownership(s);
    // s is no longer valid here

    let x = 5;
    makes_copy(x);
    // x is still valid here
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
```

---

### **Structs**
```rust
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        username: String::from("example"),
        email: String::from("example@example.com"),
        active: true,
        sign_in_count: 1,
    };
}
```

---

### **Enums**
```rust
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn move_player(dir: Direction) {
    match dir {
        Direction::Up => println!("Move Up"),
        Direction::Down => println!("Move Down"),
        Direction::Left => println!("Move Left"),
        Direction::Right => println!("Move Right"),
    }
}
```

---

### **Traits**
```rust
trait Greet {
    fn say_hello(&self);
}

struct Person;

impl Greet for Person {
    fn say_hello(&self) {
        println!("Hello!");
    }
}

fn main() {
    let p = Person;
    p.say_hello();
}
```

---

### **Error Handling**
#### **Panic**
```rust
fn main() {
    panic!("Crash and burn!");
}
```

#### **Result**
```rust
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(a / b)
    }
}
```

---

### **Collections**

#### **Vectors**
```rust
let mut v = vec![1, 2, 3];
v.push(4);
```

#### **HashMap**
```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
```

---

### **Concurrency**

#### **Threads**
```rust
use std::thread;

let handle = thread::spawn(|| {
    println!("Hello from a thread!");
});
handle.join().unwrap();
```

#### **Channels**
```rust
use std::sync::mpsc;
use std::thread;

let (tx, rx) = mpsc::channel();

thread::spawn(move || {
    tx.send("Hello").unwrap();
});

println!("{}", rx.recv().unwrap());
```

---

### **Macros**
```rust
println!("Formatted output: {}", 42);
```

---

This cheat sheet should cover most of your Rust programming needs.
