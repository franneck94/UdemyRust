fn main() {
    let mut s = String::from("foo");
    s.push_str("bar");
    s.push_str("🚀");

    println!("{}", s);

    let slice = &s[..];
    println!("{}", slice);

    // slice[0] = 'a';

    for b in s.bytes() {
        println!("{}", b);
    }

    for c in s.chars() {
        println!("{}", c);
    }
}
