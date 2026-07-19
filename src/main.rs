use std::os::windows::process;

// SESSION 3 --EXERCISE
//No. 1
#[derive(Debug)]
struct Book{
    title:String,
    author:String,
    pages:u32,
    available:bool,
}

// No. 2
enum LibraryAction {
    Checkout,
    Return,
    Donate(Book), // a donated new book
}

// No. 3
fn process_action(action: LibraryAction, book: &mut Book){
    match action {
        LibraryAction::Checkout => {
            if book.available {
                book.available = false;
                println!("'{}' has been checked out.", book.title);
            }else {
                println!("Warning: '{}' is already checked out!", book.title);
            }
        }
        LibraryAction::Return => {
            book.available = true;
            println!("'{}' has been returned.", book.title);
        }
        LibraryAction::Donate(new_book) => {
            println!("Thank you for donating '{}' by {}!", new_book.title, new_book.author);
            // Note: we can't assign '*book = new_book' because we don't own the original book.
            // The exercise just ask to print info.
        }
    }
}

fn main(){
    let mut my_book = Book {
        title: String::from("The Rust Programming Language"),
        author: String::from("Steve Klabnik & Carol Nichols"),
        pages: 550,
        available: true,
    };
    println!("Initial state: {:?}", my_book);

    // Checkout
    process_action(LibraryAction::Checkout, &mut my_book);
    println!("After checkout: {:?}", my_book);

    // Try to checkout again (should warn)
    process_action(LibraryAction::Checkout, &mut my_book);

    //Return
    process_action(LibraryAction::Return, &mut my_book);
    println!("After return: {:?}", my_book);

    //Donate
    let donated_book = Book{
        title: String::from("Rust in Action"),
        author: String::from("Tim McNamara"),
        pages: 400,
        available: true,
    };
    process_action(LibraryAction::Donate(donated_book), &mut my_book);
    // my_book remains unchanged; the donated_book has been moved into the function.
}