// Borrowing with references
fn main(){
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // Borrow from s1
    println!("Length of '{}' is {}", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // s goes out of scope, but it's just a reference, so nothing is freed