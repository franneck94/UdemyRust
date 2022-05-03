// As many immutable references as we want, but then no mutable reference
// Only one mutable reference, but then no immutable references

fn main() {
    let mut s = String::from("Jan");

    {
        let _r1 = &s;
        let _r2 = &s;

        println!("{} {}", _r1, _r2);
    }

    {
        let _r1 = &mut s;

        println!("{}", _r1);
    }

    {
        let _r1 = &s;
        let _r2 = &s;
        let _r3 = &mut s;

        println!("{} {} {}", _r1, _r2, _r3);
    }
}
