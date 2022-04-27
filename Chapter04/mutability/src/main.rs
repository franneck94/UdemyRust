fn main() {
    let _immutable = 1;
    let mut _mutable = 1;

    println!("Before mutation: {}", _mutable);

    // Ok
    _mutable += 1;

    println!("After mutation: {}", _mutable);

    // Error!
    // _immutable += 1;
    // FIXME ^ Comment out this line

    let shadowed = 1;
    {
        println!("before being shadowed: {}", shadowed);

        // This binding *shadows* the outer one
        let shadowed = "abc";

        println!("shadowed in inner block: {}", shadowed);
    }
    println!("outside inner block: {}", shadowed);

    // This binding *shadows* the previous binding
    let shadowed = 2;
    println!("shadowed in outer block: {}", shadowed);
}
