// enum Option<T> {
//     Some(T),
//     None,
// }

fn main() {
    let x: i32 = 5;
    let y = Some(5);

    println!("x + y = {}", add(x, y))
}

fn add(x: i32, y: Option<i32>) -> i32 {
    match y {
        Some(val) => x + val,
        None => x + 0,
    }
}
