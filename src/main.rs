// Session 2 Exercise
fn main(){
    let mut student_name = String::from("Pamilerin");
   change_name(&mut student_name);
    println!("{}",student_name);
}

fn change_name(student_name: &mut String){
    student_name.push_str(" Lawal");
}