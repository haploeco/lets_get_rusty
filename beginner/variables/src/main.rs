fn main() {
    // creation
    let a: i16 = 5;

    // mutability
    let mut b: i32 = 5;
    println!("b: {b}");
    b = 10;
    println!("b: {b}");

    // shadowing
    let c: i32 = 10;
    println!("c: {c}");
    let c: i32 = 20;
    println!("c: {c}");

    // scope
    let d: i32 = 30;

    {
        let d: i32 = 40;
        println!("inner d is: {d}");
    }

    println!("d is: {d}");
}
