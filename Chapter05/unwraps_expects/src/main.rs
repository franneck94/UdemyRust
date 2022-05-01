fn main() {
    let input = 1;

    let _x1 = match my_function1(input) {
        Some(v) => v,
        None => panic!(),
    };

    let _y1 = match my_function2(input) {
        Ok(v) => v,
        Err(err) => panic!("{}", err),
    };

    let _x2 = my_function1(input).unwrap();
    let _y2 = my_function1(input).unwrap();

    let _x3 = my_function1(input).expect("ERROR!!!11elf");
    let _y3 = my_function1(input).expect("ERROR!!!11elf");

    let _x4 = my_function1(input).unwrap_or("default");
    let _y4 = my_function1(input).unwrap_or("default");
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

    Err("error message".to_owned())
}
