/**
 * Vectors can only store values of the same type.
 * 
 * A vector Vec<T> can hold any type.
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

}
