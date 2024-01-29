struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = (30, 50);
    let rect2 = Rectangle {
        width: 30,
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
}


fn area_with_tuples(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_with_structs(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}