use std::io;

fn prompt_int(question: &str) -> u32 {
    println!("{}", question);

    let result: u32;
    let mut input: String = String::new();
    loop {
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        result = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a valid number!");
                input.clear();
                continue;
            }
        };

        break;
    }

    result
}

fn main() {
    let age = prompt_int("Pick an age:");
    println!("You are {} years old!", age);
}
