fn main() {
    let hello1 = String::from("Hello");

    let (hello2, len) = calculate_length(hello1);

    println!("The length of '{}' is '{}'.", hello2, len);

}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
