/// Project 5: Temperature Converter
/// Level 1: First Steps
/// Convert between Celsius and Fahrenheit
use std::io;

/// Converts Fahrenheit to Celsius
/// Formula: (°F - 32) × 5/9 = °C
fn fahrenheit_to_celsius(fahrenheit: f32) -> f32 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

/// Converts Celsius to Fahrenheit
/// Formula: °C × 9/5 + 32 = °F
fn celsius_to_fahrenheit(celsius: f32) -> f32 {
    celsius * 9.0 / 5.0 + 32.0
}

fn main() {
    println!("Input the temperature (only numbers):");
    let mut temperature = String::new();
    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read line.");
    let temperature: f32 = match temperature.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number!");
            return;
        }
    };

    println!("Pick the source unit (f, c):");
    let mut unit = String::new();
    io::stdin()
        .read_line(&mut unit)
        .expect("Failed to read line.");

    let result = match unit.trim().to_lowercase().as_str() {
        "c" | "celsius" => format!("{:.1}°F", celsius_to_fahrenheit(temperature)),
        "f" | "fahrenheit" => format!("{:.1}°C", fahrenheit_to_celsius(temperature)),
        _ => {
            println!("Please pick between \"c\" or \"f\"");
            return;
        }
    };
    println!("Result: {}", result);
}
