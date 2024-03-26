fn main() {
    call_loop();
    // call_while();
    // call_for();
}

fn call_loop() {
    let mut counter = 0;

    let result = 'looper_name: loop {
        counter += 1;
        println!("{}", counter);

        if counter == 10 {
            break 10;
        }
    };

    println!("Result: {result}")
}

fn call_while() {
    let mut counter = 0;

    'whiler_name: while counter < 10 {
        counter += 1;
        println!("{}", counter);
    }
}

fn call_for() {
    // [0, ..., 10]
    for num in 0..11 {
        println!("{}", num);
    }
}
