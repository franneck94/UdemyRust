use std::io;

fn main() {
    println!("Please enter your number: ");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read number!");

    println!("You entered: {}", guess);
}
