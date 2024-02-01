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
        // String::from("Hello {}")
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
    println!("this is new {:?}", m.call());
}

fn route(ip_kind: IpAddrKind) {}