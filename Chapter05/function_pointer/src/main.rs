fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn do_twice2<T>(f: T, arg: i32) -> i32
where
    T: Fn(i32) -> i32,
{
    f(arg) + f(arg)
}

fn main() {
    let answer1 = do_twice(add_one, 5);
    let answer2 = do_twice2(add_one, 5);

    println!("The answer is: {}", answer1);
    println!("The answer is: {}", answer2);
}
