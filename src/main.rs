use std::io;

fn main() {
    let mut input_1 = String::new();

    println!("Enter equation");

    io::stdin()
        .read_line(&mut input_1)
        .expect("Failed to read number!");

    let splitted_equation: Option<Vec<String>> = split_equation(input_1);

    match splitted_equation {
        Some(vector) => {
            let calculated_result: i32 = calculate(
                vector[0].parse::<i32>().unwrap(),
                vector[1].parse::<i32>().unwrap(),
                vector[2].to_string(),
            );
            println!("{calculated_result}");
        }

        None => println!("No equation found!"),
    }
}

// If equation can't be splitted, it's not a valid equation!
fn split_equation(equation: String) -> Option<Vec<String>> {
    let possible_operations = ["+", "-", "*", "/"];

    for operation in possible_operations {
        let mut splitted: Vec<String> = equation
            .trim()
            .split(operation)
            .map(|s| s.parse::<String>().unwrap())
            .collect();

        if splitted.len() > 1 {
            splitted.push(operation.to_owned());
            return Some(splitted);
        };
    }
    return None;
}

// TODO change from integer to decimal datatype equivalent
fn calculate(x: i32, y: i32, operation: String) -> i32 {
    match operation.as_str() {
        "+" => x + y,
        "-" => x - y,
        "*" => x * y,
        "/" => x / y,
        _ => 0,
    }
}
