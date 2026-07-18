// Session 2 Exercise
fn main(){
    let student_name = String::from("Pamilerin");
    let len = calculate_length(&student_name);
    println!("The length of '{}' is {}.", student_name, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}