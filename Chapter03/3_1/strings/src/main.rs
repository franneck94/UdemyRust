fn main() {
    let mut my_str = String::from("I 💖 Rust"); // Right Side: String Literal

    println!("{my_str}");
    my_str.push('W');
    println!("{my_str}");
    my_str.pop();
    println!("{my_str}");

    for byte in my_str.bytes() {
        println!("{byte}")
    }

    for chr in my_str.chars() {
        println!("{chr}")
    }

    for (pos, chr) in my_str.char_indices() {
        println!("{pos}: {chr}")
    }
}
