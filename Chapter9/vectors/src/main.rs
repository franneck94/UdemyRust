use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    let mut nums: Vec<i32> = vec![];

    nums.push(1);
    nums.push(2);
    nums.push(3);
    nums.push(4);

    let pop1 = nums.pop();
    println!("{:?}", pop1);

    let val0 = nums[0]; // Copy since it is a trivial type
    println!("{:?}", val0);

    let first = nums.first();

    match first {
        None => println!("Empty vector"),
        Some(first) => println!("{:?}", first),
    }

    nums.insert(0, 10);
    nums.remove(2);

    println!("{:?}", nums);
    nums.shuffle(&mut thread_rng());
    println!("{:?}", nums);
}
