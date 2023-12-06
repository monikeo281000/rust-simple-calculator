/**
 * Simple Calculator by Scarlet Hare
 * GitHub: https://github.com/ScarletHare
 * 
 * This Rust program, developed by Scarlet Hare, is a simple command-line calculator that takes three arguments:
 * [first_number] [operator] [second_number].
 * It performs basic arithmetic operations and provides clear error messages for invalid inputs.
 * The program showcases command-line argument parsing, error handling, and modular function design.
 *
 * Usage:
 *   cargo run [first_number] [operator] [second_number]
 *
 * Example:
 *   cargo run 5 + 3
 *
 * Author: Scarlet Hare
 * GitHub: https://github.com/ScarletHare
 * LinkedIn: https://www.linkedin.com/in/monikeo281000/
 * Instagram: https://www.instagram.com/moni_keo281000/
 * Username: monikeo28100
 * Date: 06 DEC 2023
 */

use std::env::args;
use std::convert::TryInto;

fn main() {
    // User collect to convert args iterator into a Vec
    let args: Vec<String> = args().skip(1).collect();

    // Check for correct number of arguments
    if args.len() != 3{
        println!("USAGE: [first_number] [operator] [second_number]");
        std::process::exit(1);
    }

    // Destructure the Vec directly to avoid unnescessary cloning
    let [first, operator, second]: [String; 3] = args.try_into().unwrap_or_else(|_| {
       println!("ERROR: Invalid argument types."); 
       std::process::exit(1);
    });

    let first_number: f64 = match first.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("ERROR: Invalid first operand.");
            std::process::exit(0);
        },
    };

    let second_number: f64 = match second.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("ERROR: Invalid second operand.");
            std::process::exit(1);
        },
    };

    // Additional check for valid operator length
    if operator.chars().count() != 1 {
        println!("ERROR: Operator must be a single character.");
        std::process::exit(1);
    }

    let operator: char = operator.trim().chars().next().unwrap_or_else(|| {
        println!("ERROR: invalid operator.");
        std::process::exit(1);
    });

    let result = operate(operator, first_number, second_number);

    println!("{}", output(first_number, operator, second_number, result));
}

fn addition(first_number: f64, second_number: f64) -> f64{
    return first_number + second_number;
}

fn subtraction(first_number: f64, second_number: f64) -> f64{
    return first_number - second_number;
}

fn multiplication(first_number: f64, second_number: f64) -> f64{
    return first_number * second_number;
}

fn division(first_number: f64, second_number: f64) -> f64{
    return first_number / second_number;
}

fn modulus(first_number: f64, second_number: f64) -> f64{
    return first_number % second_number;
}

fn operate(operator: char, first_number: f64, second_number: f64) -> f64{
    return match operator {
        '+' => addition(first_number , second_number),
        '-' => subtraction(first_number, second_number),
        '/' => division(first_number, second_number),
        '%' => modulus(first_number, second_number),
        '*' | 'x' | 'X' => multiplication(first_number, second_number),
        _ => {
          println!("ERROR: invalid operator.");
          std::process::exit(1);
        },
    };
}

fn output(first_number: f64, operator: char, second_number: f64, result: f64) -> String {
    return format!("{} {} {} = {}", first_number, operator, second_number, result);
}
