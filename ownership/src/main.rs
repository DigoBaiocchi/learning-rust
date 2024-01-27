/**
 * Ownership is the process memory management and
 * allocation between the stack and the heap
 * It frees memory (drop function) once the variables are out of scope
 * 
 * The stack stores values as last in, first out
 * 
 * The heap allocates a certain space in memory to be used and returns a pointer
 * Because the pointer to the heap is a known fixed size, it can be stored in the stack
 * 
 * Ownership rules
 * - each value has an owner
 * - there can only be one owner at a time
 * - when the owner goes out of scope, the value will be dropped
 * 
 * cloning a String type variable will create a deep copy on the heap
 * 
 * it not required to use clone() for integers because they are stored on the stack
 * No difference between deep and shallow copying when variable is created on the stack
 * 
 * Scalar variables can have a Copy trait
 * */ 

fn main() { // s and string are not valid here, it’s not yet declared                 
    let s = "hello";   // s is valid from this point forward and allocated on the stack

    println!("{s}"); // do stuff with s

    let mut string = String::from("Hello"); // String types are allocated on the heap

    string.push_str(", world!"); // Alloc in the heap allows them to be mutable

    println!("{string}"); // do more stuff with string

    let x = 5;
    let y = x; // this binds the number 5 to variables x and y in the stack

    let s1 = String::from("hello");
    let s2 = s1; // the pointer of s1 is copied here and s1 is no longer valid. s1 was moved to s2

    // println!("{}, world!", s1); compiler will throw an error

    takes_ownership(s2); // s2's value moves into the function...
                                     // ... and so is no longer valid here
    
    makes_copy(y);     // y would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use y afterward

    let ss1 = gives_ownership(); // gives_ownership moves its return value into ss1

    let ss2 = String::from("hello");     // ss2 comes into scope

    let ss3 = takes_and_gives_back(ss2);    // ss2 is moved into
                                                            // takes_and_gives_back, which also
                                                            // moves its return value into ss3
    println!("{ss1}, {ss3}");

} // this scope is now over, and s and string are no longer valid
  // Here, ss3 goes out of scope and is dropped. ss2 was moved, so nothing
  // happens. ss1 goes out of scope and is dropped.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String { // gives_ownership will move its
                                // return value into the function
                                // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and moves out to the calling function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope

    a_string  // a_string is returned and moves out to the calling function
}