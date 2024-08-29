use std::io;

fn main() {
    let possible_operations = ["*", "+", "-"];

    let mut input_1 = String::new();
    // let mut input_2 = String::new();

    println!("Enter equation");

    io::stdin()
        .read_line(&mut input_1)
        .expect("Failed to read number!");

    // let number_1: i32 = input_1
    // .trim()
    // .parse::<i32>()
    // .expect("Not a valid Number 1!");
    for operation in possible_operations {
        println!("try {operation}");
        let splitted: Vec<&str> = input_1.trim().split(operation).collect();
        // println!("{splitted}");
        // let splitted_option: Option<Vec<&str>> = splitted.collect();
        // for element in splitted {
        // assert_eq!(element, None);
        println!("{:?}", splitted);

        if splitted.len() > 1 {
            let calculated_result = calculate(
                splitted[0].parse::<i32>().unwrap(),
                splitted[1].parse::<i32>().unwrap(),
                operation,
            );

            println!("{calculated_result}");
            break;
        }

        // }
        // println!("{splitted_option}");
        // }
    }

    // Raise error as now correct equation has been found
    println!("No correct equation has been found, try again!");

    // let operation = String::from("+");

    // let result = calculate(number_1, number_2, operation);
    // println!("{}", result);
    // println!("Number1: {}; Number2: {}", input_1, input_2);
}

fn calculate(x: i32, y: i32, operation: &str) -> i32 {
    match operation {
        "+" => x + y,
        "-" => x - y,
        "*" => x * y,
        "/" => x / y,
        _ => 0,
    }
    // return x + y;
}
