fn main() {
    let mut nums: Vec<i32> = vec![];

    nums.push(1);
    nums.push(2);
    nums.push(3);

    let _popped_value1 = nums.pop();
    let _popped_value2 = nums.pop();
    let _popped_value3 = nums.pop();
    let _popped_value4 = nums.pop();

    match _popped_value4 {
        Some(_) => {}
        None => println!("Vec is already empty!"),
    }

    let mut nums2: Vec<i32> = vec![5, 2, 1, 3, 4];

    match nums2.get(5) {
        Some(_) => {}
        None => println!("INDEX NOT VALID"),
    }

    nums2.sort();
    nums2.reverse();

    println!("Sorted vec: {:?}", nums2)
}
