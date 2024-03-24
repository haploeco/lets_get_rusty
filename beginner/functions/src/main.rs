fn main() {
    let test: i32 = my_functions(22);
    println!("test: {test}");
}

fn my_functions(x: i32) -> i32 {
    println!("my_function called with: {x}");
    let y: i32 = 10;
    y
}
