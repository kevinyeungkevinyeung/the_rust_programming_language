use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str where T: Display {
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    // 1. Each parameter that is a reference gets its own lifetime parameter

    // 2. If therre is exactly one input lifetime parameter, that lifetime is
    //    assigned to all output lifetime parameters

    // 3. If there are multiple input lifetime parameters, but one of them is
    //    &self or &mut self the lifetime of self is assigned to all output lifetime parameters

    let r;
    {
        let x = 5;
        r = &x;
    }
    println!("r: {}", r);

    let string1 = String::from("abcd");
    let string2 = String::from("xyz");

    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is {}", result);

    // example 2
    // let string1 = String::from("abcd");
    // {
    //     let string2 = String::from("xyz");
    //     let result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {}", result); // string 2 lifetime doesn't last until this point
}

// &i32 --> a reference
// &'a i32 --> a reference with an explicit lifetime
// &'a mut i32 --> a mutable reference with an explicity lifetime

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // x and y might have different lifetime
    // need to know x or y lifetime to be used as return lifetime

    // the return lifetime will have the shorter lifetime of either x or y
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
