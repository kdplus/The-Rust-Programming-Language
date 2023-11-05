fn main() {
    let number = 3;

    // Rust will not automatically try to convert non-Boolean types to a Boolean.
    // You must be explicit and always provide if with a Boolean as its condition.
    // if number { is not allowed
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number != 0 {
        println!("number was something other than zero");
    }

    let number = 6;

    // multiple condition with else if
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // Because if is an expression,
    // we can use it on the right side of a let statement to assign the outcome to a variable
    let condition = true;
    let number = if condition { 5 } else { 6 };

    // the values that have the potential to be results from each arm of the if must be the same type
    // let number = if condition { 5 } else { "six" }; will leads to error

    println!("The value of number is: {number}");
}
