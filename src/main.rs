// SESSION 003
// Tuple struct: named tuple with fields
struct Color(i32, i32, i32);
struct Point(i32,i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);

// Unit-like struct: no fields, used for traits
struct AlwaysEqual;
let subject = AlwaysEqual;