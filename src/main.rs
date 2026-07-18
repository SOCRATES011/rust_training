//First small exercise
fn main(){
    let name = "Dare Akinlade"; // number 1
    let mut age:u8 = 35; //number 2

    // number 3
    for number in (1..6).rev(){
        println!("Number: {} '  ' square of {} is: {}",number,number, number*number);
    }

    //number 4
    let status = if age >= 18 {"adult"} else {"minor"};
    println!(" {} is {}years old therefore, I am an {}",name, age, status);

}