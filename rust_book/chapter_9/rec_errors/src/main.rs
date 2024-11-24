use std::fs::{self, File};
use std::io::ErrorKind;
use std::io::{self, Read};

fn main() {
    let filepath = "hello.txt";
    let greeting_file_result = File::open(&filepath);

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(&filepath) {
                Ok(fc) => fc,
                Err(error) => panic!("Problem creating the file: {error}"),
            },
            other_error => {
                panic!("Problem opening the file {other_error:?}");
            }
        },
    };
    // matches are useful, but primitive and this is very verbose. Another way is using closures (see chapter 13 for details)

    let greeting_file = File::open(&filepath).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(&filepath).unwrap_or_else(|error| {
                panic!("Problem creating the file: {error}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });

    let greeting_file = File::open(&filepath).unwrap();
    let greeting_file = File::open(&filepath).expect("Hello.txt should be included in the project");
    // most rustacians use expect rather than unwrap to provide better feedback in a case something that should give an error gave an error

    
}


fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
} // error propagation is so common in rust that it provides a shortcut operator '?' to make it easier, as followed in the next function

fn read_username_from_file_simplified() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_more_simplified() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
} // chaining the ? operator
// ? return an error to the calling code if it happends or the expression results in the value of the Ok(value)

fn read_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}