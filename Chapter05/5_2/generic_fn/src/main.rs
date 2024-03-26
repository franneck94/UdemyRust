use std::cmp::PartialOrd;

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = [34, 50, 25, 100, 65];
    let result1 = largest(&number_list);
    println!("Result1: {result1}");

    let char_list = ['y', 'm', 'a', 'q'];
    let result2 = largest(&char_list);
    println!("Result2: {result2}");
}
