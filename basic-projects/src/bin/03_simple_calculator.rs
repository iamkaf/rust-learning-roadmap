/// A simple, looping command-line calculator.
///
/// It prompts for an operation and two numbers, then prints the result.
/// Handles invalid input and division by zero. Type 'q' to quit.

use std::io;

#[derive(PartialEq)] // Magic so we can compare enums
enum OP {
    NONE,
    ADD,
    SUBTRACT,
    MULTIPLY,
    DIVIDE,
    MODULUS,
}

fn main() {
    println!("Welcome to the world's best calculator!");

    loop {
        println!(
            "Pick operation: [+ add] [- subtract] [* multiply] [/ divide] [% modulus] [q quit]"
        );

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line.");

        let operation: OP = match choice.trim() {
            "+" => OP::ADD,
            "-" => OP::SUBTRACT,
            "*" => OP::MULTIPLY,
            "/" => OP::DIVIDE,
            "%" => OP::MODULUS,
            "q" => {
                println!("Goodbye!");
                break;
            }
            _ => OP::NONE, // For any other input, we get NONE
        };

        if operation == OP::NONE {
            // try again!
            continue;
        }

        println!("Type first number:");

        choice.clear(); // need to reset the string to use it again
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line.");

        let first_number: i32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That's not a valid number!");
                continue;
            }
        };

        println!("Type second number:");

        choice.clear(); // need to reset the string to use it again
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line.");

        let second_number: i32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That's not a valid number!");
                continue;
            }
        };

        let result: i32 = match operation {
            OP::ADD => first_number + second_number,
            OP::SUBTRACT => first_number - second_number,
            OP::MULTIPLY => first_number * second_number,
            OP::DIVIDE => {
                if second_number == 0 {
                    println!("Error: Cannot divide by zero!");
                    continue; // Skip the rest of this loop iteration
                }
                first_number / second_number
            }
            OP::MODULUS => {
                if second_number == 0 {
                    println!("Error: Cannot divide by zero!");
                    continue;
                }
                first_number % second_number
            }
            OP::NONE => panic!("Huh!?"),
        };

        println!("Result: {result}");
    }
}
