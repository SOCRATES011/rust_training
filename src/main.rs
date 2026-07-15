//Variables, mutability and shadowing
fn main() {
    let x = 5; 
    let mut y = 20;
    y = 10;
    println!("x: {}, y: {}", x, y);

// shadowing rebind the same name, even changing type
let x = "now a string";
println!("x: {}", x);

}
