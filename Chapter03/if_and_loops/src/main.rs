fn main() {
    // checker();
    // looper();
    // whiler();
    forer();
}

fn checker() {
    let num = 1;

    if num > 10 {
        println!("True");
    } else if num == 1 {
        println!("Equal");
    } else {
        println!("False");
    }
}

fn looper() {
    let mut num = 0;

    'counter: loop {
        num += 1;
        println!("{}", num);

        if num == 50 {
            break 'counter;
        }
    }
}

fn whiler() {
    let mut num = 0;

    while num < 50 {
        num += 1;
        println!("{}", num);
    }
}

fn forer() {
    let nums = 0..51; // 0..=50

    for num in nums {
        println!("{}", num);
    }
}
