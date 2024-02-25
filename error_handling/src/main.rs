/*
 * Rust will run by default panic for unrecoverable errors, like
 * trying to access index 99 of a vector that only has 3 characters.
 * panic can also be added to the code.
 * 
 * 
 * 
 * 
 * 
 **/ 

fn main() {
    let v = vec![1,2,3];

    v[99];
}
