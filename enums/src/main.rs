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
 * 
 * 
 * Match control flow - 
 * allows you to compare a value against a series of patterns
 * and then execute the code when has a match.
 * 
 * If statements evaluate the conditions as boolean. In match control flow
 * it can also be executed by matching the type. At the end, the flow will
 * execute the first arm that matches the pattern.
 * 
 * They can also bind to the parts of the values that match the pattern.
 * See coin example below for more clarification.
 * 
 * We can also use match control flow with Option<T>.
 * It will check if the type matches
 * 
 * The arms' patterns must cover all possibilities otherwise it won't compile.
 * Matches in Rust are exhaustive, we must exhaust every last possibility in order
 * to make the code valid and compile.
 * 
 * We can use the special word other or the character _ to set default behaviors for results 
 * that are not specified.
 * 
 * We are able to print Option values using a match control flow without the need of using unwrap().
 * 
 * We can use if let to do a more concise control flow, specially when we only need 
 * to match one pattern and ignore all others.
 * It's good to keep in mind that using if let instead of match has trade-offs that 
 * must be considered. A more concise control flow might not catch all
 * different scenarios that the application might encounter.
 * 
 * We can also use else if the if let control flow to execute something for all other patterns.
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

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    NewOrleans,
    NewYork,
    Washington,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => {
            println!("the value is: {}", i);
            Some(i + 1)
        },
    }
}

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
    let quarter = value_in_cents(Coin::Quarter(UsState::NewOrleans));
    let penny = value_in_cents(Coin::Penny);

    println!("The value of a penny is: {}", penny);
    println!("The value of a quarter is: {}", quarter);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("The value of six is: {}", six.unwrap());

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: i8) {}

    // concise control flow with if let
    let config_max = Some(3u8);
    // using match control flow
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
    // using if let
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    let coin = Coin::Nickel;
    let mut count = 0;

    // using match control flow
    // match coin {
    //     Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    //     _ => count += 1,
    // }

    // using if let else
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}

fn route(ip_kind: IpAddrKind) {}
