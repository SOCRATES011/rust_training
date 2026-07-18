// Clone (deep copy)
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone(); // s2 is a deep copy of s1
    println!("s1 = {}, s2 = {}", s1, s2); // both valid
    
}