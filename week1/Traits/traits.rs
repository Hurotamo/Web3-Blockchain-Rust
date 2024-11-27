trait Describable {
    fn describe(&self) -> String;
}

struct Person {
    name: String,
}

impl Describable for Person {
    fn describe(&self) -> String {
        format!("Person named {}", self.name)
    }
}

fn main() {
    let p = Person {
        name: String::from("Alice"),
    };
    println!("{}", p.describe());
}
