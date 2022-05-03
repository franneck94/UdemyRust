/// Borrowing in a scope
/// As many immutable references as we want, but then no mutable reference
/// Only one mutable reference, but then no immutable references

fn main() {
    // let s = String::from("hello");
    let mut s = String::from("hello");

    {
        let _r1 = &s;
        let _r2 = &s;

        println!("{}, {}", _r1, _r2);
    }

    {
        let _r3 = &mut s;

        println!("{}", _r3);
    }

    {
        let _r4 = &s;
        let _r5 = &s;
        let _r6 = &mut s;

        println!("{}, {}, {}", _r4, _r5, _r6);
    }
}
