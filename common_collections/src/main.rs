/**
 * VECTORS
 * Vectors can only store values of the same type.
 * A vector Vec<T> can hold any type.
 * 
 * The following code will throw an error because when you create a vector
 * some space in memory is allocated for it, and when a new
 * number is added to the vector, the currect memory is deallocated and new
 * memory is allocated for the updated vector. So when you try to print
 * the variable first, it would be trying to read deallocated memory. 
 * let mut v = vec![1, 2, 3, 4 , 5];
 * let first = &v[0];
 * v.push(6);
 * println!("The first element is: {first}");
 * 
 * Enums can be used as a way to add different types to a vector.
 * The enum, with different types, would be added to the vector and all values
 * would be the type enum.
 * 
 * Like any other struct, a vector is freed when it goes out of scope.
 * 
 * 
 * STRINGS
 * Rust only has one string type, which is the string slice str that is usually
 * seen in its borrowed form &str.
 * The String type is a growable, mutable, owned, UTF-8 encoded string type.
 * 
 * Many of the same operations available with Vectors are also available with String
 * becasue String is actually implemented as a wrapper around a vector of bytes
 * with some extra guarantees, restrictions and capabilities.
 * 
 * Use push_str concatenate strings and push to add chars to strings.
 * 
 * If we use the + sign to concatenate strings and assign the result to a variable. 
 * Everything that was not passed as a reference will have the ownership transferred 
 * to the new variable.
 * If we use format! function to concatenate strings, the string variables will its
 * ownership transferred.
 * */ 

fn main() {
    let mut v: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let third = &v[2];
    println!("The third element is {third}");

    let third_again = v.get(2);
    match third_again {
        Some(third_again) => println!("The third element again is {third_again}"),
        None => println!("There is no third element."),
    }

    // loop through vector and change each number
    // we need to use * deference to get the value i before we can use the +=
    let mut vector = vec![100, 32, 87];
    for i in &mut vector {
        *i += 50;
        println!("{i}");
    }
    let second = vector.get(1);
    match second {
        Some(&second) => println!("{}", second),
        None => println!("Nothing"),
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    let data = "initial contents";

    let s = data.to_string();
    // OR
    let s2 = String::from("initial contents");

    println!("{} or {}", s, s2);

    let mut t = String::from("foo");
    t.push_str("bar");
    println!("{}", t);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    // s2 ownership is transferred to s1 here so it can't be used anymore
    s1.push_str(s2);
    println!("{}", s2);

    let s3 = "Haha";
    let s4 = s1 + &s3;
    println!("{s4}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");

}
