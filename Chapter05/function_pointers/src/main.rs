fn adder(x: i32) -> i32 {
    x + 1337
}

fn get_offset(x: i32) -> i32 {
    if x > 0 {
        return 1337;
    }

    -1337
}

fn adder2(x: i32, f: fn(i32) -> i32) -> i32 {
    x + f(x)
}

fn adder3<T>(x: i32, f: T) -> i32
where
    T: Fn(i32) -> i32,
{
    x + f(x)
}

fn main() {
    let result = adder2(10, get_offset);
    println!("Result: {}", result);

    let result2 = adder3(10, get_offset);
    println!("Result2: {}", result2);
}
