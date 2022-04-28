fn main() {
    let s = format!("Hello {}, Nice to meet you", "Jan");

    print!("{}", s);
    println!("{}", s);

    let e = "Error message!";

    eprint!("{}", e);
    eprintln!("{}", e);

    let s2 = format!("Hello {}", "Jan");
    let s3 = "Nice to meet you";
    println!("{}, {}", s2, s3);
    println!("{0}, {1}", s2, s3);

    // Since rust 1.58
    let name = "Jan";
    let s4 = format!("Hello {name}, Nice to meet you");
    print!("{}", s4);
}
