use std::io;

fn main() {
    println!("Simple Rust Calculator");
    
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut operator = String::new();

    println!("Enter first number:");
    io::stdin().read_line(&mut input1).expect("Failed to read line");
    let num1: f64 = input1.trim().parse().expect("Please enter a valid number");

    println!("Enter an operator (+, -, *, /):");
    io::stdin().read_line(&mut operator).expect("Failed to read line");
    let operator = operator.trim();

    println!("Enter second number:");
    io::stdin().read_line(&mut input2).expect("Failed to read line");
    let num2: f64 = input2.trim().parse().expect("Please enter a valid number");

    let result = match operator {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => {
            if num2 == 0.0 {
                println!("Error: Cannot divide by zero");
                return;
            } else {
                num1 / num2
            }
        }
        _ => {
            println!("Invalid operator");
            return;
        }
    };

    println!("Result: {}", result);
}
