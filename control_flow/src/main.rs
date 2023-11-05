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

    // returning values from loops
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    // Loop Labels to Disambiguate Between Multiple Loops
    // Loop labels must begin with a single quote
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

    // Conditional Loops with while
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // Looping Through a Collection with for
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    // while is error prone in this case due to the size of array may be modified
    // It’s also slow, because the compiler adds runtime code to perform the conditional check
    // of whether the index is within the bounds of the array on every iteration through the loop.
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    // use for to countdown
    // The way to do that would be to use a Range, provided by the standard library,
    // which generates all numbers in sequence starting from one number and ending before another number
    // it's left inclusive and right exclusive: (start..end) => [start, end)
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
