// SESSION 3 - No. 4 and 5
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

// Option enum - no null (use the standard library Option<T>:
enum Option<T> {
    Some(T),
    None,
}

let some_number = Some(5);
let absent_number: Option<i32> = None;

// Accessing value: pattern matching or combinators
match some_number {
    Some(x) => println!("Value: {}", x);
    None => println!("Nothing");
}

// Using if let for concise handling
if let Some(x) = some_number {
    println!("Value is {}", x),
}