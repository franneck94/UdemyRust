fn forer() {
    let nums = 0..=10;

    for num in nums {
        println!("{}", num);
    }
}

fn while_letter() {
    let mut nums = (0..=10).into_iter();

    while let Some(num) = nums.next() {
        println!("{}", num);
    }
}

fn main() {
    forer();
    while_letter();
}
