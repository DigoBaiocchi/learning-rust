/**
 * Enums can hava multiple types within
 * 
 * One advantage of enums over structs is that
 * you can group multiple types within an enum.
 * In order to accomplish the same with structs,
 * it would be required one struct for each type
 * 
 * Enums can also have functions.
 * In order to use them we use impl as separate block
 * with the same name as the enum
 * 
 * Options - they are rust's alternative to null value
 * Type Option<i8> is different than type i8. 
 * We wouldn't be able to sum two numbers where one if of type
 * Option<i8> and the other i8 because i8 it will always be a number
 * but Option<i8> could not be a number.
 * If it's required to add these two numbers, we have to convert
 * Option<i8> to i8. It can converted with method unwrap(self)
 * */ 

enum IpAddrKind {
    v4(String),
    v6(String),
}

enum IpAddr {
    v4(u8, u8, u8, u8),
    v6(String),
}

enum Message {
    Quit,
    Move { x: i32, y:i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
        match self {
            Message::Quit => println!("This quits"),
            Message::Move { x, y } => println!("x is: {} and y is: {}", x, y),
            Message::Write(text) => println!("Text message: {}", text),
            Message::ChangeColor(r, g, b) => println!("r is: {}, g is: {} and b is: {}", r, g, b),
        }
    }
}

// Representational of Option under the hood, not required to specify the following code
// enum Option<T> {
//     None,
//     Some(T),
// }

fn main() {
    let four = IpAddrKind::v4;
    let six = IpAddrKind::v6;

    route(IpAddrKind::v4(String::from("127.0.0.1")));
    route(IpAddrKind::v6(String::from("::1")));

    let home = IpAddr::v4(127, 0, 0, 1);
    let loopback = IpAddr::v6(String::from("::1"));

    let m = Message::Write(String::from("Hello"));
    m.call();

    let mut some_number = Some(5);
    let number = 5;
    assert_eq!(some_number.unwrap(), number);
    println!("Value of some_number is: {}", some_number.unwrap());
    let some_char = Some('e');

    let absent_number: Option<i32> = None;
}

fn route(ip_kind: IpAddrKind) {}