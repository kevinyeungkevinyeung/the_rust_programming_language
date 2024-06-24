use std::ops::Add;


#[derive(Debug, PartialEq)]
struct Point{
    x:i32,
    y:i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other:Point) -> Point{
        Point{
            x:self.x + other.x,
            y:self.y + other.y,
        }
    }
}

// trait Add<Rhs=Self> {
//     type Output;

//     fn add(self, rhs:Rhs) -> Self::Output;
// }

pub trait Iterator<T> {

    fn next(&mut self) -> Option<T>;
}

struct Counter {}

impl Iterator<u32> for Counter{

    fn next(&mut self) -> Option<u32>{
        Some(0)
    }
}

impl Iterator<u16> for Counter{

    fn next(&mut self) -> Option<u16>{
        Some(0)
    }
}

struct Millimeters(u32);

struct Meters(u32);

impl Add<Meters> for Millimeters{
    type Output = Millimeters;

    fn add(self, other:Meters) -> Millimeters{
        Millimeters(self.0 + (other.0 * 1000))
    }
}


fn main() {
     assert_eq!(
        Point{x:1,y:0} + Point {x:2, y:3},
        Point{x:3,y:3}
     );

}