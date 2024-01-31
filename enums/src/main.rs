/**
 * Enums can hava multiple types within
 * */ 

enum IpAddrKind {
    v4(String),
    v6(String),
}

enum IpAddr {
    v4(u8, u8, u8, u8),
    v6(String),
}

fn main() {
    let four = IpAddrKind::v4;
    let six = IpAddrKind::v6;

    route(IpAddrKind::v4(String::from("127.0.0.1")));
    route(IpAddrKind::v6(String::from("::1")));

    let home = IpAddr::v4(127, 0, 0, 1);
    let loopback = IpAddr::v6(String::from("::1"));
}

fn route(ip_kind: IpAddrKind) {}