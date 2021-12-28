fn main() {
    lost_ownership();
}

// 4.1  What is Ownership
    // Each value in Rust has a variable that’s called its owner.
    // There can only be one owner at a time.
    // When the owner goes out of scope, the value will be dropped.


fn lost_ownership() {
    let mut s = String::from("test");
    println!("hello {}", s);
}

