fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        };
    };
    println!("The result is {result}.");

    // loop label
    let mut count = 0;
    'counting_up: loop {
        println!("Count = {count}");
        let mut remaining = 10;

        loop {
            println!("Remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    // while loop
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("The value is : {}", a[index]);

        index += 1;
    }

    // for loop
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LEFT OFF !!");
}
