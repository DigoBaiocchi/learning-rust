/**
 * References are guarantee to point to a valid value of a particular type 
 * for the life of that reference
 * 
 * Use & to specify a reference to parameter type in a function
 * and to the variable you are passing as param to the function
 * 
 * If the value reference that is passed in the function params needs to be modified,
 * it needs to specify mut after & (reference) before the type in the function param
 * and in the variable that will be passed as param.
 * 
 * The action of creating a reference is called borrowing
 * 
 * A big restriction in this approach is this approach is that a mutable reference cannot
 * be shared between two variables like the following:
 * let mut s = String::from("hello");
 * 
 * let r1 = &mut s;
 * let r2 = &mut s;
 * 
 * println!("{}, {}", r1, r2);
 * 
 * You can't borrow one reference to more once at a time.
 * 
 * This prevents data races at compile time.
 * Data race has these 3 behaviors:
 * - two or more pointers access the same data at the same time
 * - at least one of the pointers is being used to write to the data
 * - there's no mechanism being used to synchronize access to the data
 * Data races causes undefined behavier
 * 
 * Can't have a reference be immutable and mutable at the same time
 * let mut s = String::from("hello");
 * 
 * let r1 = &s; //no problem
 * let r2 = &s; //no problem
 * let r3 = &mut s; //BIG PROBLEM
 * 
 * println!("{}, {}, and {}", r1, r2, r3);
 * 
 * instead we can do the following:
 * let mut s = String::from("hello");
 * 
 * let r1 = &s; //no problem
 * let r2 = &s; //no problem
 * 
 * println!("{}, and {}", r1, r2); // r1 and r2 will be dropped after this
 * 
 * let r3 = &mut s; //no problem
 * 
 * println!("{}", r1);
 * 
 
 * Rust also prevents dangling references, when we try to borrow a value 
 * when there's nothing to be borrowed from, like the following:
 * 
    fn dangle() -> &String { // dangle returns a reference to a String
        let s = String::from("hello"); // s is a new String

        &s // we return a reference to the String, s
    } // Here, s goes out of scope, and is dropped. Its memory goes away.
      // Danger!
 * 
 * The solution for this case is returning the String directly with no &.
 * In this case ownership is moved out and nothing is deallocated
 * 
 * */ 

fn main() {
    let hello1 = String::from("Hello");

    let (hello2, len) = calculate_length_withouth_reference(hello1);

    println!("The length of '{}' is '{}'.", hello2, len);

    let s1 = String::from("Other Hello");

    let len2 = calculate_length_with_reference(&s1);

    println!("The length of '{}' is '{}'.", s1, len2);

    let mut s = String::from("hello");

    change(&mut s);

    println!("The value of s is '{}'", s)
}

fn calculate_length_withouth_reference(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

fn calculate_length_with_reference(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

fn change(some_string: &mut String) {
some_string.push_str(", world");
}