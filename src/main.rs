// Move semantics in Rust means that when a variable is assigned to another variable,
// the ownership of the value is transferred (moved) to the new variable.
fn main(){
    let s1 = String::from("hello"); // s1 owns the string
    let s2 = s1; // ownership of the string is moved to s2
    //println!("{}", s1); // s2 goes out of scope here and the memory is freed
    println!("{}", s2);
}