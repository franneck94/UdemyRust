use colored::*;
use rand::Rng;
use std::io;

fn main() {
    let secret_num = rand::thread_rng().gen_range(0..=10);

    println!("Welcome to the Guessing Game!\n");

    loop {
        println!("Please enter a number in the range [0, 10]:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read number!");

        let guessed_num = match guess.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => return,
        };

        if guessed_num == secret_num {
            println!("{}", "Your guess is correct".green());
            break;
        } else {
            if secret_num < guessed_num {
                println!("{}", "Your guesses number is too big!".red());
            } else {
                println!("{}", "Your guesses number is too small!".red());
            }
        }
    }
}
