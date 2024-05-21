fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // result in -1

    // remainder
    let remainder = 43 % 5;

    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");

    let five_hundred = tup.0;

    let six_point_four = tup.1;

    let one = tup.2;

    // array
    // fixed length with same type
    // allocated on the stack rather than heap
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];

    another_function();
}

fn another_function() {
    println!("another function!!!!")
}
