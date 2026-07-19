// SESSION 3
// Enums: are algebraic data types, each variant can carry data.
#[derive(Debug)]
enum IpAddrKind {
    V4(String),        // variant with attached String
    V6(String),
}

let home = IpAddrKind::V4(String::from("127.0.0.1"));
let loopback = IpAddrKind::V6(String::from("::1"));

// Better example: different data per variant
enum Message {
    Quit,
    Move {x: i32, y: i32},  // named fields
    Write(String),
    ChangeColor(i32, i32, i32),   //tuple-like
}

// Enums can also have methods
impl Message {
    fn call(&self) {
        //method body
    }
}