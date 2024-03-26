fn first_word(s: &String) -> &str {
    for (idx, char) in s.char_indices() {
        if char == ' ' {
            return &s[..idx];
        }
    }

    &s[..]
}

fn main() {
    let s = String::from("Jan Schaffranek");

    let slice1 = &s[0..3];
    let slice2 = &s[4..];

    println!("{slice1}");
    println!("{slice2}");

    let word1 = first_word(&s);
    println!("{word1}");
}
