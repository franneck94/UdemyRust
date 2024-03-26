// enum Option<T> {
//     Some(T),
//     None,
// }

fn main() {
    let x: u32 = 5;
    let y: Option<u32> = Some(5);

    println!("x + y = {}", add(x, y));

    let x2: u32 = 5;
    let y2: Option<u32> = None;

    println!("x2 + y2 = {}", add(x2, y2));
}

fn add(a: u32, b: Option<u32>) -> u32 {
    match b {
        Some(b_value) => a + b_value,
        None => a + 0,
    }
}
