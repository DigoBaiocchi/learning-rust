// using #[derive(Debug)] before a struct helps us debugging code printing the struct values
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

/**
 * impl implement the method using the desired struct.
 * All functions defined within an impl block are called
 * associated functions.
 * 
 * Associated functions that aren't methods are ofent used for 
 * constructors that will return a new instance of the struct.
 * To access need to use :: after the struct name.
 * See square implementation below
 * */ 
impl Rectangle {
    fn area_with_method(&self) -> u32 {
        self.width * self.height
    }

    // we can choose to give a method the same name as one of struct's fields.
    // Rust differenciates between rect2.width and rect2.width()
    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, rectangle: &Rectangle) -> bool {
        self.width > rectangle.width && self.height > rectangle.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let scale = 3;
    let rect1 = (30, 50);
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 50
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!(
        "The area of the rectangle 1 is {} square pixels.",
        area_with_tuples(rect1)
    );

    println!(
        "The area of the rectangle 2 is {} square pixels.",
        area_with_structs(&rect2)
    );

    println!(
        "The area of the rectangle 2 using a method is {} square pixels. And is {} that the width is greater than 0",
        rect2.area_with_method(), rect2.width()
    );

    println!("Rect2 is {:#?}", &rect2);
    // dbg! is another way to print structs to help with debugging
    dbg!(&rect2);

    println!("Can rect2 hold rect3? {}", rect2.can_hold(&rect3));

    let sq = Rectangle::square(3);
    println!("Width of the square is: {} and height of the square is: {}", sq.width, sq.height);
}

// makes the code more concise but not as clear
fn area_with_tuples(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// makes the code not as concise tuples but still clear
fn area_with_structs(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}