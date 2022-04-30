fn main() {
    let s = "Jan Schaffranek".to_owned();

    // take_ownership1(&s); // immutable move
    // take_ownership2(&s); // immutable move

    take_reference1(&s); // immutable ref
    take_reference2(&s); // immutable ref

    take_reference1("Jan"); // immutable ref
    take_reference2("Jan"); // immutable ref
}

fn take_ownership1(p: String) {
    println!("{}", p);
}

fn take_ownership2(p: String) {
    println!("{}", p);
}

fn take_reference1(p: &str) {
    println!("{}", p);
}

fn take_reference2(p: &str) {
    println!("{}", p);
}
