use std::fs::{ self, File };
use std::io::ErrorKind;
use std::io::{ self, Read };

fn main() {
    let greeting_file_result = File::open("Hello.text");

    let greet_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem opening the file: {:?}", error);
        }
    };

    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) =>
            match error.kind() {
                // get a error enum back for matching
                ErrorKind::NotFound =>
                    match File::create("hello.txt") {
                        Ok(fc) => fc,
                        Err(e) => panic!("Problem createing the file: {:?}", e),
                    }
                other_error => {
                    panic!("Problem opening the file: {:?}", other_error);
                }
            }
    };

    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file {:?}", error);
        }
    });

    let greeting_file = File::open("hello.txt").unwrap(); // if return ok, it will return the file, else it will panic

    let greeting_file = File::open("hello.txt").expect(
        "hello.txt should be included in this project"
    );
}

fn read_username_from_file() -> Result<String, io::Error> {
    //let mut username = String::new();
    //File::open("hello.txt")?.read_to_string(&mut username)?; // ? if error return error, else return the file
    //Ok(username)

    //? operator can only be used in a function that returns Result or Option
    fs::read_to_string("hello.txt")
}
