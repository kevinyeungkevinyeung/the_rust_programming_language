struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let user_1 = build_user(String::from("kevin.yeung@yahoo.com"), String::from("kevin.yeung"));

    let user_2 = User {
        email: String::from("another@hotmail.com"),
        ..user_1
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;

    let rec_1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area of rectangle is {} square pixels.", area(&rec_1));

    println!("rect1 is {:?}", rec_1);

    println!("The area of rectangle is {} square pixels.", &rec_1.area());

    let rec_2 = Rectangle {
        width: 10,
        height: 5,
    };

    let rec_3 = Rectangle {
        width: 20,
        height: 15,
    };

    println!("Can rect_3 hold rect_2? {}", rec_3.can_hold(&rec_2));
}

// fn build_user(email:String, username: String) -> User {
//     User {
//         active:true,
//         username:username,
//         email:email,
//         sign_in_count:1,
//     }
// }

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
