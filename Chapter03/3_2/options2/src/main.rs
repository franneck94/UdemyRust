// enum Option<T> {
//     Some(T),
//     None,
// }

fn main() {
    let x1: Option<u32> = Some(5);
    let y1: Option<u32> = Some(5);

    println!("x + y = {}", add1(x1, y1));
    println!("x + y = {}", add2(x1, y1));

    let x2: Option<u32> = Some(5);
    let y2: Option<u32> = None;

    println!("x2 + y2 = {}", add1(x2, y2));
    println!("x2 + y2 = {}", add2(x2, y2));

    let x3: Option<u32> = None;
    let y3: Option<u32> = Some(5);

    println!("x3 + y3 = {}", add1(x3, y3));
    println!("x3 + y3 = {}", add2(x3, y3));

    let x4: Option<u32> = None;
    let y4: Option<u32> = None;

    println!("x4 + y4 = {}", add1(x4, y4));
    println!("x4 + y4 = {}", add2(x4, y4));
}

fn add1(a: Option<u32>, b: Option<u32>) -> u32 {
    match a {
        Some(a_value) => match b {
            Some(b_value) => a_value + b_value,
            None => a_value,
        },
        None => match b {
            Some(b_value) => b_value,
            None => 0,
        },
    }
}

fn add2(a: Option<u32>, b: Option<u32>) -> u32 {
    let a_value = if let Some(a_value) = a { a_value } else { 0 };
    let b_value = if let Some(b_value) = b { b_value } else { 0 };

    a_value + b_value
}
