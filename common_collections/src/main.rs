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
 * 
 * We can't index string like hello[0] because indexing in Rust would only return
 * the UTF-8 code and not the actual first character of hello in this case.
 * 
 * We can get a string slice specifying the range of bytes that should return from
 * the string, like hello[0..4]. In this case, it will return the bytes that were 
 * stored from 0 to 3.
 * 
 * The best way to iterate through strings is to be explicit about whether you
 * want characters or bytes. Using the chars or bytes method.
 * 
 * Be aware that valid Unicode scalar values may be made up of more than 1 byte,
 * so this will influence on how you should get a slice of a string.
 * 
 * HASH MAPS
 * The type HashMap<K, V> stores a mapping of keys of type K to values of type V using a 
 * hashing function, which determines how it places these keys and values into memory.
 * 
 * Hash maps are not part of the standard library so in order to use we need to specify the following:
 * use std::collections::HashMap;
 * 
 * Like vectors, hash maps are homnogeneous: all of the keys must have the same type as each other,
 * and all of the values must have the same type.
 * 
 * For types that implement the Copy trait, like i32, the values are copied into the hash map.
 * For owned values like String, the values will be moved and the hash map will be the owner
 * of those values. So after the ownerhip is transferred we can't use the string variables anymore.
 * 
 * HashMap uses, by default, a hashing function called SipHash. It is safe but not the fastest algo available.
 * We can switch to another function by specifying a different hash.
 * */

fn main() {
    let mut v: Vec<i32> = Vec::new();
    let _v2 = vec![1, 2, 3];

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

    let _row = vec![
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

    let _s = format!("{s1}-{s2}-{s3}");

    let hello = "Здравствуйте";

    let s = &hello[0..4];
    println!("{s}");

    for c in hello.chars() {
        println!("{c}");
    }

    for c in hello.bytes() {
        println!("{c}");
    }

    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("{}", score);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
    
    // overwriting a value
    scores.insert(String::from("Blue"), 25);
    
    println!("{:?}", scores);
    
    // if key already exists keep old value, if new add as new value
    scores.entry(String::from("Red")).or_insert(35);
    scores.entry(String::from("Blue")).or_insert(35);

    println!("{:?}", scores);

    let text = "Hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

}
