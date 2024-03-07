/*
 * Rust will run by default panic for unrecoverable errors, like
 * trying to access index 99 of a vector that only has 3 characters.
 * panic can also be added to the code.
 * 
 * The result enum is defined as having Ok and Err variants.
 * 
 * When a function's implementation calls something that might fail,
 * instead of handling the error within the function itself, we can
 * return the error to the calling code so that it can decide what
 * to do. This is known as propagating the error
 * 
 * The ? symbol can also be used to replace the match statement.
 * It should be placed after the statement we want to do.
 * 
 * There's no need to call Err when using ? symbol because this symbol
 * calls the from function and the error type received is converted into
 * the error type defined in the return type of the current function.
 * 
 * The ? symbol can only be used in function whose return type is compatible 
 * with the value of ? is used on. Because of this, it can't be used 
 * in a main function. Unless we change the return type of the main function to
 * for example, if we want to return a Result, the main return type would be 
 * like Result <(), Box<dyn Error>>. The Box<dyn Error> mean "any kind of error".
 * 
 * When we use ? in a function that returns an Option, if the value is None,
 * the None will be returned early from the function at that point. If the 
 * value is Some, the value inside the Some is the resulting value of the
 * expressionand and the function continues.
 * 
 * 
 **/

use std::{fs::{self, File}, io::{self, ErrorKind, Read}}; 

fn main() {
    let _v = vec![1,2,3];

    // v[99];

    let greeting_file_result = File::open("hello.txt");

    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(_e) => panic!("Problem creating the file: {:?}", error),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    // logic above can be rewritten without match statements
    let _greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // we can also use unwrap for a very short version and still catches the error
    let _greeting_file = File::open("hello3.txt").unwrap();

    // using expect will display our error message
    let _greeting_file = File::open("hello4.txt")
        .expect("hello.txt should be included in this project");
}

// propagating error example
fn _read_username_from_file() -> Result<String, io::Error> {
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
}

// logic above can be rewritten using the ? symbil
fn _read_user_from_file_2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn _read_user_from_file_3() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username);

    Ok(username)
}

fn _read_user_from_file_4() -> Result<String, io::Error> {
    fs::read_to_string("Hello.text")
}

// using ? with function that returns an Option
fn _last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}