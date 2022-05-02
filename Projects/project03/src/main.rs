use std::num::ParseIntError;

fn total_cost(quantity_str: &str) -> Result<u32, ParseIntError> {
    let cost_per_item: u32 = 10;

    let quantity = quantity_str.parse::<u32>()?;

    Ok(cost_per_item * quantity)
}

fn main() {
    let mut tokens = 100;
    let user_input = "8";

    let cost = total_cost(user_input).unwrap_or(0);

    if cost > tokens {
        println!("Not enough tokens!")
    } else {
        tokens -= cost;
        println!("You have {} tokens", tokens);
    }
}
