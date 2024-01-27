/**
 * Structs are like a general template for the type and 
 * instances fill in that template with data to create
 * values of the type.
 * 
 * All the instance must be mutable if we need to change at least 
 * one value in an instance
 * 
 * User field init allows shorthand syntax when user function parameteres 
 * have the same name as keys
 * 
 * When using the struct from one variable into another,
 * only the variables that have Copy trait (stored in the stack)
 * will be able to be accessed
 * 
 * Tuple structs don't have name fields as regular structs
 * they only have the type of the required fields
 * Still dot notation can be used to access an element in the tuple
 * 
 * You can also define structs that don't have any fields
 * They are called unit-like structs and they behave similarly to ()
 * 
 * It's also possible to store references to data owned by something else
 * but in order to do it requires lifetimes. Lifetimes will be approached 
 * in chapter 10
 * */ 

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let mut user1 = User {
        active: true, // still valid because has bool has Copy trait
        username: String::from("someusername123"),
        email: String::from("someemail@email.com"), // can't be accessed anymore because was moved to user4
                                                    // and String type doesn't have Copy trait
        sign_in_count: 1, // still valid because has bool has Copy trait
    };

    user1.username = String::from("anotherusername");

    let user2 = build_user(String::from("testemail@email.com"), String::from("testuser"));

    // let user3 = User {
    //     active: user1.active,
    //     username: String::from("myusername"),
    //     email: user1.email,
    //     sign_in_count: user1.sign_in_count,
    // };

    // OR

    let user4 = User {
        username: String::from("mynewusername"),
        ..user1
    };

    println!("user1: {} and user2: {} and user4: {}", user1.username, user2.username, user4.username);

    let black = Color(0, 1, 0);
    let origin = Point(0, 0, 0);
    println!("{}", black.1);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username /* same as username: username, */,
        email /* same as email: email, */,
        sign_in_count: 1,
    }
}