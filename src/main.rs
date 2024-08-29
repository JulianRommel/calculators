use std::io;

fn main() {
    let mut input_1 = String::new();
    let mut input_2 = String::new();

    println!("Enter first number");
    io::stdin()
        .read_line(&mut input_1)
        .expect("Failed to read number!");

    println!("Enter second number");
    io::stdin()
        .read_line(&mut input_2)
        .expect("Failed to read number!");

    let number_1: i32 = input_1
        .trim()
        .parse::<i32>()
        .expect("Not a valid Number 1!");
    let number_2: i32 = input_2
        .trim()
        .parse::<i32>()
        .expect("Not a valid Number 2!");
    // println!("{number_1}");
    let result = calculate(number_1, number_2);
    println!("{}", result);
    // println!("Number1: {}; Number2: {}", input_1, input_2);
}

fn calculate(x: i32, y: i32) -> i32 {
    return x + y;
}
