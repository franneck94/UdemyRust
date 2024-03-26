// By default, a variable has read/own permissions (RO) on its data
// If a variable is annotated with let mut, then it also has the write permission (W)
// The key idea is that references can temporarily remove these permissions.

// As many immutable references as we want, but then no mutable reference
// Only one mutable reference, but then no immutable references
// References are non-owning pointers, because they do not own the data
// immutable references are also called shared references
// mutable references are also called unique references

fn main() {
    let mut s = String::from("Jan");

    {
        let _r1 = &s; // immutable ref
        let _r2 = &s; // immutable ref

        println!("{} {}", _r1, _r2);
    }

    {
        let _r1 = &mut s; // mutable ref

        println!("{}", _r1);
    }

    {
        // let _r1 = &s; // immutable ref
        // let _r2 = &s; // immutable ref
        // let _r3 = &mut s; // mutable ref

        // println!("{} {} {}", _r1, _r2, _r3);
    }
}
