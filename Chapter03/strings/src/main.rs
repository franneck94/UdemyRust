fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &character) in bytes.iter().enumerate() {
        if character == b' ' {
            return &s[..i];
        }
    }

    return s;
}

fn main() {
    let mut hello = String::from("💖 Rust");

    println!("{}", hello);
    hello.push('w');
    println!("{}", hello);

    for b in hello.bytes() {
        println!("{}", b);
    }

    for c in hello.chars() {
        println!("{}", c);
    }

    let word = first_word(&hello);
    println!("First word: {}", word);
}
