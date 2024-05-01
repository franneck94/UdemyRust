# Foreword

I will present my solution to all of the exercises in this course in a video.  
However you can, but you don't have to, try to get to the solution by yourself.  
This is optional to you.

# Guessing Game Exercise

## Objective

Create a guessing game where the player has a limited number of attempts to guess the secret number.  
Additionally, add features such as displaying the number of remaining attempts and providing hints to guide the player's guesses.

As a start we use the code from Chapter 2

```rust
fn main() {
    println!("Please enter a number:");

    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read number!");

    println!("You entered: {user_input}");
}
```

## Instructions

### Initialize Attempts

Initialize a variable to hold the maximum number of attempts the player is allowed.  

### Game Loop

Implement a loop to include a counter for the number of attempts made by the player.  
The loop should terminate when the player either guesses the correct number or exhausts all attempts.

### Display Remaining Attempts

Implement a mechanism to display the number of remaining attempts to the player after each guess.

### Completion Message

Upon the game ending, display a message indicating whether the player won or lost.

**Happy coding!**
