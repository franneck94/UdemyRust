fn main() {
    // Immutable
    {
        let s = String::from("Jan");

        let s2 = &s;

        println!("{s}");
        println!("{s2}");
    }

    // Mutable
    {
        let mut s = String::from("Jan");

        let s2 = &mut s; // RWO

        println!("{s2}");

        println!("{s}");
    }
}
