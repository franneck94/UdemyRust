fn main() {
    // let x = 1337; // immutable per default - Read Only afterwards
    let mut x = 1337; // mutable  - Read and Write

    println!("The current value of x is: {x}");

    x = -10;

    println!("The current value of x is: {x}");
}
