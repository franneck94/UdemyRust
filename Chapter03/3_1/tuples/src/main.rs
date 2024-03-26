// Tuples: Fixed length, different dtypes possible

fn main() {
    let special_tpl = (); // unit

    let mut tpl = (500, "hi", true);

    // println!("{tpl}"); // std::fmt::Display
    println!("{tpl:?}"); // std::fmt::Debug

    let (mut x, y, z) = tpl;
    println!("{x}");
    println!("{y}");
    println!("{z}");

    x = -100;

    println!("{tpl:?}"); // std::fmt::Debug

    tpl.0 = -100;

    println!("{tpl:?}"); // std::fmt::Debug
}
