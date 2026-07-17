// Control flow: if, loop, while, for.
fn main(){
    let number = 7;

    if number < 5 {
        println!("less than 5");
    } else if number == 5 {
        println!("equal to 5");
    } else {
        println!("greater than 5");
    }
    // 'if' is an expression - it returns a value
    let result = if number % 2 == 0 { "even" } else {"odd"};
    println!("{} is {}", number, result);

    // loop (infinite) - use break to exit
    let mut counter = 0;
    let final_value = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; //break returns a value
        }
    };
    println!("Final value: {}", final_value);

    // while
    let mut n = 3;
    while n != 0 {
        println!("{}!", n);
        n -= 1;
    }
    println!("LIFTOFF");

    //for with range (most common)
    let arr = [10,20,30,40,50,60,70];
    for element in arr.iter(){
        println!("the value is: {}", element);
    }

    for number in (1..4).rev(){ // range 1..4 is 1,2,3
        println!("{}!", number);
    }
}