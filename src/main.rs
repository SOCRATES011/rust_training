// Session 2 Exercise
fn main(){
    let student_name = String::from("Pamilerin");
    let first_name = &student_name;
    let last_name = &student_name.clone();
    println!("my full name is {} {}", first_name, last_name);
}
