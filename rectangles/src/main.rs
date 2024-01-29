// using #[derive(Debug)] before a struct helps us debugging code printing the struct values
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = (30, 50);
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 50
    };

    println!(
        "The area of the rectangle 1 is {} square pixels.",
        area_with_tuples(rect1)
    );

    println!(
        "The area of the rectangle 2 is {} square pixels.",
        area_with_structs(&rect2)
    );

    println!("Rect2 is {:#?}", &rect2);
    // dbg! is another way to print structs to help with debugging
    dbg!(&rect2);
}

// makes the code more concise but not as clear
fn area_with_tuples(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// makes the code not as concise tuples but still clear
fn area_with_structs(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}