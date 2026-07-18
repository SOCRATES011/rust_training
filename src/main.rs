// Ownership
fn main(){
    let s = String::from("hello"); // s owns the string
    println!("{}", s);
    // s goes out of scope here and the memory is freed
}