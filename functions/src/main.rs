/**
 * Every function parameter must have the type declared in original function
 * 
 * statements in a function don't return a value - use semicolon at the end
 * but expressions do - don't use semicolon at the end
 * 
 * you can't bind a variable to a statement like let x = y = 6
 * but you can do like:
 * let x = { 
 *      let y = 6;
 *      y = 1 
 * }
 * 
 * expressions don't semicolon at the end
 * if you add a semicolon at the end of an expression, it will turn into a statement
 * 
 * no need to use the word return to specify a returned value in a function
 * but type must be declared after an arrow ->
 * 
 * returns can be used if you want to return early in a function ??
 * */ 

fn main() {
    println!("Hello, world!");

    let x = { // begin of main statement
        let y = 6; // statement
        y + 1 // expression
    }; // end of main statement

    println!("The value of x is: {x}");

    another_function();
    again_another_function(5);
    print_labeled_measurement(5, 'h');
    let is_five = is_five(5);
    println!("Value of is_five is: {is_five}");
}

fn another_function() {
    println!("Another function.");
}

fn again_another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn is_five(x: i32) -> bool {
    if x == 5 {
        true
    }
    else {
        false
    }
}