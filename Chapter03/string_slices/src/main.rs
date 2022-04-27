fn main() {
    let s = String::from("This is my text");

    let word = first_word(&s);

    println!("First word: {}", word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &character) in bytes.iter().enumerate() {
        if character == b' ' {
            return &s[0..i];
        }
    }

    return s;
}
