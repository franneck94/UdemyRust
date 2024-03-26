// Copy per Default:
// All the integer types, such as u32.
// The Boolean type, bool, with values true and false.
// All the floating-point types, such as f64.
// The character type, char.
// Tuples, if they only contain types that also implement Copy (see above)

#[derive(Debug)]
struct Circle {
    pub radius: f32,
}

fn main() {
    let tpl = (1.0, Circle { radius: 1.0 });

    println!("{tpl:?}");

    let (mut x, mut y) = tpl;
    println!("{x:?}");
    println!("{y:?}");

    x = 2.0;
    y = Circle { radius: 2.0 };

    // println!("{tpl:?}");

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}
