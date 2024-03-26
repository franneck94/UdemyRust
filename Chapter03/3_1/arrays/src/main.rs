// Arrays: Fixed length, only same dtypes

fn main() {
    let array = [1, 2, 3]; // stack

    println!("{array:?}");
    println!("{}", array[0]);
    println!("{}", array[1]);
    println!("{}", array[2]);

    let mut array2 = [1337; 5]; // list comprehension
    println!("{array2:?}");
    println!("{}", array2[0]);
    println!("{}", array2[1]);
    println!("{}", array2[2]);
    println!("{}", array2[3]);
    println!("{}", array2[4]);

    let mut x = array2[0];
    x = 10;

    println!("{array2:?}");

    array2[0] = 10;

    println!("{array2:?}");
}
