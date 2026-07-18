// Session 2 Exercise
fn main(){
    let mut student_name = String::from("Pamilerin");
    {
        let full_name = &mut student_name;
        full_name.push_str(" Esther");
        println!("My names are: {}", full_name);
    }
    // Now new mutable reference can be created
    let all_name = &mut student_name;
    all_name.push_str(" Lawal");
    println!("All my names are: {}", all_name);
}

