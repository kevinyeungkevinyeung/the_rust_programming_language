enum Language {
    English,
    Spanish,
    Russian,
    Japanese,
}

struct Point {
    x: i32,
    y: i32,
}

struct Point3 {
    x: i32,
    y: i32,
    z: i32,
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum Message2 {
    Hello { id: i32 },
}

fn main() {
    let Language = Language::English;

    match Language {
        Language::English => println!("Hello world!"),
        Language::English => println!("Hello world in spanish!"),
        Language::English => println!("Hello world in russian !"),
        _ => println!("Unsupported language!"),
    }

    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x = 5;

    match x {
        1..=5 => println!("one thrrough five"),
        _ => println!("something else"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    let p = Point { x: 0, y: 7 };
    match p {
        Point { x, y: 0 } => {
            println!("On the x axis at {}", x)
        }
        Point { x: 0, y } => {
            println!("On the y axis at {}", y)
        }
        Point { x, y } => {
            println!("On neither axis: ({}, {})", x, y)
        }
    }

    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("Quit");
        }
        Message::Move { x, y } => {
            println!("MOve to x:{}, y:{}", x, y);
        }
        Message::Write(text) => {
            println!("Text message: {}", text)
        }
        Message::ChangeColor(r, g, b) => {
            println!("change color: red {}, green {}, and blue {}", r, g, b)
        }
    }

    // let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    // match msg {
    //     Message::ChangeColor(Color::Rgb(r, g, b)) => {
    //         println!("Change color: red {}, green {}, and blue {}", r, g, b);
    //     }
    //     Message::ChangeColor(Color::Hsv(h, s, v)) => {
    //         println!("Change color: hue {}, saturation {}. and vale {}", h, s, v);
    //     }
    //     _ => (),
    // }

    foo(3, 4);

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("setting is {:?}", setting_value);

    let origin = Point3 { x: 0, y: 0, z: 0 };

    match origin {
        Point3 { x, .. } => println!("x is {}", x),
    }

    let numbers = (2, 4, 6, 8);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }

    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case , x = {:?}", x),
    }

    let msg = Message2::Hello { id: 5 };

    match msg {
        Message2::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message2::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message2::Hello { id } => {
            println!("Found some other id: {}", id)
        }
    }
}

fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter:{}", y);
}
