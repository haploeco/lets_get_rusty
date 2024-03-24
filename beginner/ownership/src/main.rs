fn main() {
    // heap allocated data is moved by default. To clone it, use the clone
    // method

    let s1 = String::from("Rust"); // heap allocated string
    let s2 = s1.clone();

    println!("s1 is: {s1}");
    println!("s2 is: {s2}");

    // Primitive types that are stored on the stack are cloned and not moved
    // ie: int, float, bool, char, etc.
    let x: i32 = 10;
    let y = x;
    println!("x is: {x}");
    println!("y is: {y}");
} // s1 is dropped
