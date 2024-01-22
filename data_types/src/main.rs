/**
 * Interger can be signed (i + total bits to be used) or unsigned (u + total bits to be used) 
 * Interger can be written as number literals (decimals, hex, octal, binary and byte (u8 only))
 * 
 * Floating-point types are f32 (single precision) and f64 (double precision)
 * 
 * Rust supports addition, subtraction, division, multiplication and remainder
 * In division truncates toward zero to the nearest integer
 * 
 * Booleans are one byte in size
 * 
 * Char type is 4 bites
 * 
 * Tuple type has a fixed length
 * 
 * Arrays must always have the same type for all elements and also has a fixed length
 * */ 

fn main() {
    let mut x: u8 = 5;
    // can't reassign a variable to number that has more bits than previously defined
    // x = 555; ---> number has more than 8 bits
    println!("Value of x is: {x}");
    
    x = 12;
    
    println!("New value of x is: {x}");

    let sum = 5 + 10;
    println!("Value of sum is: {sum}");
    
    let difference = 95.5 - 4.3;
    println!("Value of difference is: {difference}");
    
    let product = 4 * 30;
    println!("Value of product is: {product}");
    
    let quotientf32: f32 = 56.7 / 32.2;
    println!("Value of quotient single precision is: {quotientf32}");
    let quotientf64: f64 = 56.7 / 32.2;
    println!("Value of quotient double precision is: {quotientf64}");
    
    let truncated = -5 / 3;
    println!("Value of truncated is: {truncated}");
    
    let remainder = 1005 % 2;
    println!("Value of remainder is: {remainder}");

    let t = true;
    println!("Value of t is: {t}");

    let c = 'z';
    println!("The value of c is: {c}");

    let name = "Rodrigo";
    println!("The value of name is: {name}");

    let tup = (400, 82.1, 1);

    let (x, y ,z) = tup;

    let four_hundred = tup.0;

    println!("The value of x is: {x}, y is: {y} and z is: {z}. Value of four_hundred is: {four_hundred}");

    let array_a = [1, 2, 3];
    let first_element = array_a[0];

    let array_b = [3; 5]; // this creates -> [3,3,3,3,3];
    let second_element = array_b[1];

    println!("The first element of array_a is: {first_element} and second of array_b is: {second_element}");


}
