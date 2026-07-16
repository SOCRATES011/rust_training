// Basic types and type annotations
fn main() {
    //Integers: i8, i16, i32, i64, i128, isize(signed) and u8, u16, u32, u64, u128, usize(unsigned)
    let age: u8 = 30; //unsigned 8-bit (0-255)
    let temperature = -5; //i32 by default (signed 32-bit)

    //Floats: f32, f64 (default)
    let pi = 3.14159;   //f64 by default

    //Booleans
    let is_rust_fun = true; //bool

    //Characters (4 bytes, Unicode scalar values)
    let heart_eyed_cat = '😻'; //char

    //Tuples (fixed-size, can contain different types)
    let tup: (i32, f64, char) = (400, 7.4, 'A');
    let (x,y,z) = tup; //destructuring
    println!("x = {}, y = {}, z = {}", x, y, z);
    println!("first = {}", tup.0); //accessing tuple elements by index

    // Arrays (fixed-size, same type, stack-allocated)
    let arr: [i32; 6] = [1,2,3,6,5,4];
    println!("arr[2] = {}", arr[2]); //accessing array elements by index
    let x = 5; 
    let mut y = 20;
    y = 10;
    println!("x: {}, y: {}", x, y);

}
