fn main() {
    let s = String::from("Jan Schaffranek");

    // take_ownership1(s); // value moved here
    // take_ownership2(s); // value used here after move

    borrows1(&s); // immutable references
    borrows2(&s); // immutable references
}

fn take_ownership1(p1: String) {
    println!("{p1}");
}

fn take_ownership2(p2: String) {
    println!("{p2}");
}

fn borrows1(p1: &String) {
    println!("{p1}");
}

fn borrows2(p2: &String) {
    println!("{p2}");
}
