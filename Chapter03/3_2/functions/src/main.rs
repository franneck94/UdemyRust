fn main() {
    let num = 2;

    let result = my_function(num);

    println!("Result: {result}");
}

fn my_function(input: i32) -> i32 {
    println!("{input}");

    // return input * 2;
    input * 2
}
