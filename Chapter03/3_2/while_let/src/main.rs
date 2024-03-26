fn while_letter1() {
    let mut nums = 0..=10;

    // while (Some(num) == nums.next())
    while let Some(num) = nums.next() {
        println!("{num}");
    }
}

fn while_letter2() {
    let mut nums = [1, 2, 3].into_iter();

    while let Some(num) = nums.next() {
        println!("{num}");
    }
}

fn main() {
    while_letter1();
    while_letter2();
}
