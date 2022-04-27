// Tuples: Fixed length, different dtyps possible

fn main() {
    let mut _tpl = (500, "hi", true);

    println!("{}", _tpl.0);
    println!("{}", _tpl.1);
    println!("{}", _tpl.2);

    let (x, y, z) = _tpl;

    println!("{}", x);
    println!("{}", y);
    println!("{}", z);

    _tpl.0 = 400;

    println!("{:?}", _tpl);
}
