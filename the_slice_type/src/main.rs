/**
 * String slices is a reference to part of a String.
 * It creates a reference to a section of a string using range syntax
 * 
 * We use slices in string literals because they are immutable
 * 
 * Arrays can also have slices storing an array reference in a variable for example
 * */ 

fn main() {
    let mut s = String::from("Hello world Hey");

    let word = first_word_length(&s);

    let hello = &s[..word]; // string slice
    // same as let hello = &s[0..word];
    let world = &s[6..]; // string slice
    // same as let world = &s[6..11];

    // `first` works on slices of `String`s, whether partial or whole

    // let first = first_word(&s);
    // let first = first_word(&s[..]);
    let first = first_word(&s[0..6]);
    let second = second_word(&s[..]);

    let my_string_literal = "Hello World when I am awake";

    let other = &first_word(&my_string_literal[0..6]);

    println!("{}, {}", hello, world);
    println!("{}, {}, {}", first, second, other);

    let index = 5;
    let selected_word = select_word(&my_string_literal, index);

    println!("Word located in index {} is: {}", index, selected_word);

    let a = [1,2,3,4];

    let slice = &a[1..3];

    assert_eq!(slice, &[2,3]);

    s.clear();
}

fn first_word_length(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn select_word(s: &str, index: usize) -> String {
    let bytes = s.as_bytes();
    let mut result = String::from("");
    let mut word_position_counter = 1;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            word_position_counter += 1;
            continue;
        }

        if word_position_counter == index {
            result.push_str(&s[i..i+1]);
        } 

         if word_position_counter > index {
            break;
         }   
        
    }

    result
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]; // slice of first word
            // same as return &s[0..i];
        }
    }

    &s[..] // slice of the entire string
}

fn second_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            let first_letter = i + 1;
            return &s[first_letter..];
        }
    }

    &s[..]
}