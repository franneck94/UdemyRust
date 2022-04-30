fn forer() {
    let nums = 0..11;

    for num in nums {
        println!("{}", num);
    }
}

fn whiler() {
    let mut nums = (0..11).into_iter();

    while let Some(num) = nums.next() {
        println!("{}", num);
    }
}

fn main() {
    forer();
    whiler();
}
