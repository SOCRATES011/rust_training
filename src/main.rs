// SESSION 003
// Defining a struct
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main(){
    let user1 = User {
        email:String::from("socrates@gmail.com"),
        username: String::from("socratesakinlade"),
        active: true,
        sign_in_count: 1,
    };
    println!("{} has signed in {} times.", user1.username, user1.sign_in_count);
    
    //mutable instance can allow change of field(s)
    let mut user2 = User {
        email:String::from("apesinobabodunrin@gmail.com"),
        username: String::from("apesinakinlade"),
        active: true,
        sign_in_count: 5,
    };
    user2.email = String::from("apesinakinlade@gmail.com");
    println!("{} new email is: {}", user2.username, user2.email);

    // Copy fields from another instance:
    let user3 = User{
        email: String::from("socratesakinlade@gmail.com"),
        ..user1 // copy remaining fields from user1, but note: username is String, so user1.username is moved
    };
    // user1.username is no longer valid if moved; but user1.active, sign_in_count are copied (Copy types).
    println!(" {} sign in {} times, with {} still {}", user3.username, user3.sign_in_count, user3.email, user3.active)

}