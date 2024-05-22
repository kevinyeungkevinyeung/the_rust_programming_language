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

    // function
    another_function(12, 'm');

    println!("{}", five());

    // control flow
    let number = 3;
    if number < 3 {
        println!("Less than 3");
    } else if number == 3 {
        println!("Equal 3");
    } else {
        println!("Greater to 3");
    }

    // let statement
    let equal = if number == 3 { true } else { false };
    println!("{}", equal);
}

fn another_function(value: i32, unit_label: char) {
    println!("The measurement is {value} {unit_label}");
}

fn five() -> i32 {
    5
}
