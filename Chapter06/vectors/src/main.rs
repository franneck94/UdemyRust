fn main() {
    let mut nums: Vec<i32> = vec![];

    nums.push(1);
    nums.push(2);
    nums.push(3);

    let popped_value1 = nums.pop();
    let popped_value2 = nums.pop();
    let popped_value3 = nums.pop();
    let popped_value4 = nums.pop();

    match popped_value3 {
        Some(val) => println!("Popped value: {}", val),
        None => println!("Popped none value"),
    }

    // nums.insert(1, 10);

    let mut nums2 = vec![5, 2, 1, 3, 4];

    match nums2.get(5) {
        Some(val) => println!("Value: {}", val),
        None => {}
    }

    // println!("{}", nums2[5]);

    nums2.sort();
    println!("{:?}", nums2);
}
