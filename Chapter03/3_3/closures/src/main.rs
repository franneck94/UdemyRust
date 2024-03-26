fn my_function(inp: i32) -> i32 {
    inp * 2
}

fn my_function2() {
    let num = 2;
    let closure = |inp: i32| -> i32 { inp * 2 };
    let ret = closure(num);
    println!("{}", ret);
}

fn main() {
    let num = 2;
    let ret = my_function(num);
    println!("{}", ret);

    my_function2();
}
