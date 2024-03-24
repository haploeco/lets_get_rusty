fn main() {
    // if/else

    let a: i32 = 6;

    if a > 5 {
        println!("bigger than 5");
    } else if a > 3 {
        println!("bigger than 3");
    } else {
        println!("smaller or equal to 3");
    }

    let b: i32 = if a > 5 { 1 } else { -1 };
    println!("b: {b}");

    let mut count: i32 = 0;
    let mut inner_count: i32 = 0;

    loop {
        while inner_count < 15 {
            println!("inner loop: {inner_count}");
            inner_count += 1;
        }
        if count == 0 {
            count += inner_count;
        }


        if count > 20 {
            break;
        }

        println!("outer loop: {count}");

        count += 1;
    }

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    for element in a {
        println!("element: {element}");
    }
}
