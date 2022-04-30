fn main() {
    let number = 13;

    match number {
        1 => {
            println!("One!");
            println!("One!");
        }
        2 | 3 | 5 | 7 | 11 => println!("Prime!"),
        _ => println!("None!"),
    }

    let boolean = true;

    let binary = match boolean {
        false => 0,
        true => 1,
    };

    println!("{} -> {}", boolean, binary);
}
