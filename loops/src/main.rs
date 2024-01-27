/**
 * Loops can be executed with loop, while and for
 * 
 * loop is infinite and it requires a break to exit
 * loop can be binded to a variable
 * when a loop has an inner loop, loop labels help inner loop to break the outer loop
 * 
 * while loops will loop for as long as its condition is true
 * 
 * for loops are more suitable to loop fixed ranges, like from 0 to 5 or through all elements in an array
 * */ 

fn main() {
    // infinite loop
    // loop {
    //     println!("again!");
    // }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is: {result}");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    let mut number = 3;

    while number != 0 {
        println!("{number}");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}");
    }

    println!("{}", get_fibonacci(19));

    let celsius_temperature = 5;
    let fahrenheit_temperature = celsius_to_fahrenheit(celsius_temperature);

    println!("{celsius_temperature} celsius in fahrenheit is: {fahrenheit_temperature}");
}

fn celsius_to_fahrenheit(x: i32) -> i32 {
    (x * 9/5) + 32
}

fn get_fibonacci(x: usize) -> i32 {
    let fibonacci = [0,1,1,2,3,5,8,13,21,34,55,89,144,233,377,610,987,1597,2584,4181];

    if x > 19 {
        println!("Select a number between 0 and 19");
        0
    } else {
        fibonacci[x]
    }
}