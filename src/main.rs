// Session 3 -- No. 7
// the 'Result' enum - error handling without exceptions
use std::fs::File;

fn main(){
    let f = File::open("hello.txt");
    match f {
        Ok(file) => println!("File opened: {:?}", file),
        Err(error) => println!("Problem opening file: {:?}", error),
    }
}