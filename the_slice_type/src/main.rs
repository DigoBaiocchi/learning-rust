/**
 * String slices is a reference to part of a String.
 * It creates a reference to a section of a string using range syntax
 * 
 * We use slices in string literals because they are immutable
 * */ 

fn main() {
    let mut s = String::from("Hello world");

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

    let my_string_literal = "Hello World";

    let third = &my_string_literal;

    println!("{}, {}", hello, world);
    println!("{}, {}, {}", first, second, third);

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