fn main() {
    let x = five();

    println!("The value of x is: {x}");

    let x = plus_one(five());

    println!("The value of x is: {x}");

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    print_labled_measurement(5, 'h');
}

fn print_labled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
