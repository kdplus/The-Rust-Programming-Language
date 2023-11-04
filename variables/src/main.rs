use std::io;

fn main() {
    // Mutable
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Const
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // Shadowing
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // Shadowing allow us to change both type and value
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The number of space in spaces is {spaces}");

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division, integer division truncates toward zero to the nearest integer
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    let t = true;

    let f: bool = false; // with explicit type annotation

    // character, note that we specify char literals with single quotes, as opposed to string literals, which use double quote
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    // compound types
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // destructure a tuple value
    let (x, y, z) = tup;

    println!("The value of tuple is: {x}, {y}, {z}");

    // access tuple element by using period (.)
    let five_hundred = tup.0;

    let six_point_four = tup.1;

    let one = tup.2;

    println!("Get element value from tup: {five_hundred}, {six_point_four}, {one}");

    // arrays in Rust have a fixed length
    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5]; // initialize an array to contain the same value
    println!(
        "The value of array a is: {}, {}, {}, {}, {}",
        a[0], a[1], a[2], a[3], a[4]
    );

    let a = [1, 2, 3, 4, 5];
    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
}
