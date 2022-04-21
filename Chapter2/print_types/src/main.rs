fn main() {
    let s = format!("Hello {}, I am a formatted text", "Jan");

    print!("{}", s);
    println!("{}", s);

    let e = "Error message!";

    eprint!("{}", e);
    eprintln!("{}", e);

    let s2 = format!("Hello {}", "Jan");
    let s3 = "I am a formatted text";
    println!("{}, {}", s2, s3);
    println!("{0}, {1}", s2, s3);
}
