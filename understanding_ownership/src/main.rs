fn main() {
    let mut s1 = String::from("Hello");

    s1.push_str(", world!"); // push_str() appends a literal to a string

    println!("{s1}");

    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");

    // heap vs stack
    let s = String::from("hello");

    takes_ownership(s); // s's value moves into the function and so is no longer valid after

    let x = 5; // x would move into the function, but i32 is Copy
               //, so it's okay to still use x afterward

    makes_copy(x);

    // reference
    let s1 = String::from("Hello");
    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");

    // mutable reference
    let mut s2 = String::from("Hello");
    change(&mut s2);

    println!("{s2}");

    // slice
    let mut s = String::from("hellow world");

    let word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""
               // word still has the value 5 here, but there's no more string that
               // we could meaningfully use the value 5 with. word is now totally invalid!
}

fn change(some_string: &mut String) {
    some_string.push_str(" , world");
}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what it refers to, the string is not dropped

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}
fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    return &s[..];
}
