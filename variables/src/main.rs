/**
 * const can be declared in any scope.
 * It is only allowed to set const to a constant expression 
 * and not non-constant value
 */ 
const TOTAL_VALUE: u32 = 10 + 6;

fn main() {
    // adding mut to a let variable make them reassignable to another variable to the same type
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    
    // compiler gives a warning in the following, need to use shadowing
    // {
    //     x = 6 *2;
    //     println!("The inner value of x is: {x}");
    // }
    println!("The value of x is: {x}");
    
    // variable shadowing
     
    let y = 2;

    let y = y * 2;

    // complier doesn't give a warning in the following
    {
        let y = y *2;
        println!("The value of inner y is: {y}");
    }

    println!("The value of y is: {y}");

    println!("The total value is: {TOTAL_VALUE}");

    // variable shadowing are good if you need to reuse the same variable for different types
    let spaces = "    ";
    let spaces = spaces.len();

    println!("Total space of spaces is: {spaces}");

    // it doesn't allow you to do the same with mut, see below:
    // let mut spaces = "    ";
    // spaces = spaces.len();
}
