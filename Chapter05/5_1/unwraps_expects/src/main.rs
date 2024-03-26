// unwrap method is a shortcut method implemented just like the match expression
// If the Result value is the Ok variant, unwrap will return the value inside the Ok
// If the Result is the Err variant, unwrap will call the panic! macro for us

// the expect method lets us also choose the panic! error message

fn main() {
    let input = -1;

    // let x1 = match my_function1(input) {
    //     Some(v) => v,
    //     None => panic!(),
    // };
    // println!("x1: {x1}");

    // let y1 = match my_function2(input) {
    //     Ok(v) => v,
    //     Err(err) => panic!("{}", err),
    // };
    // println!("y1: {y1}");

    // let x2 = my_function1(input).unwrap();
    // println!("x2: {x2}");
    // let y2 = my_function2(input).unwrap();
    // println!("y2: {y2}");

    // let x3 = my_function1(input).expect("ERROR!!!");
    // println!("x3: {x3}");
    // let y3 = my_function2(input).expect("ERROR!!!");
    // println!("y3: {y3}");

    let x4 = my_function1(input).unwrap_or("default");
    println!("x4: {x4}");
    let y4 = my_function2(input).unwrap_or("default");
    println!("y4: {y4}");
}

fn my_function1(val: i32) -> Option<&'static str> {
    if val >= 0 {
        return Some("abc");
    }

    None
}

fn my_function2(val: i32) -> Result<&'static str, String> {
    if val >= 0 {
        return Ok("abc");
    }

    Err(String::from("error message"))
}
