Rust's ownership system ensures that there is exactly one owner for any piece of data, which leads to memory safety and avoids data races. Ownership rules are:

Each value in Rust has a variable that is its "owner".
A value can only have one owner at a time.
When the owner of a value goes out of scope, the value is dropped and the memory is freed.
