use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();
        println!("Please input your guess.");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        if guess.contains("q") {
            println!("Bye!");
            break;
        }

        println!("You guessed: {}", guess.trim());

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal => {
                println!("The secret number is {secret_number}.");
                println!("You win!!!");
                break;
            }
        }
    }
}
