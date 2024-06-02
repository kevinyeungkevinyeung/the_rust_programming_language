// struct Point<T> {
//     x: T,
//     y: T,
// }

// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }

// impl Point<f64> {
//     fn y(&self) -> f64 {
//         &self.x
//     }
// }

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let largest = get_largest(number_list);
    println!("The largest number is {}", largest);

    let number_list = vec![100, 200, 300, 500, 1000, 100];
    let largest = get_largest(number_list);
    println!("The largest number is {}", largest);

    // let p1 = Point { x: 5, y: 10 };
    // p.x();
    // let p2 = Point { x: 5.0, y: 10.0 };
    // p1.y();
    //let p3 = Point {x:5,y:10.0}; won't work, because both x and y need to be the same type T

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: "c" };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

fn get_largest<T: PartialOrd + Copy>(number_list: Vec<T>) -> T {
    let mut largest = number_list[0];
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    largest
}
