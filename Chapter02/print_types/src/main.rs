fn main() {
    let s = "Hello World!";

    print!("{}\n", s);
    println!("{}", s);

    eprint!("{}\n", s);
    eprintln!("{}", s);

    let name = "Daniel";
    let formatted_string = format!("Hello {name} nice to meet you!");

    println!("{formatted_string}");
    println!("{}", formatted_string);
}
