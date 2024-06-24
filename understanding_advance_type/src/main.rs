use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write(f, "[{}]", self.0.join(", "))
    }
}

struct Age(u32);

struct ID(u32);

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("word")]);

    println!("w = {}", w);

    type Kilometers = i32;
    
    let x:i32 = 5;
    let y:Kilometers = 5;

    println!("x + y = {}", x+y);

    // type alias
    type Thunk = Box<dyn Fn() + Send + 'static>;

    let f: Thunk = Box::new(||println!("hi"));

    fn takes_long_type(f:Thunk) {
        // --snip--
    }

    fm returns_long_type() -> Thunk {
        // --snip--
    }

    
}
