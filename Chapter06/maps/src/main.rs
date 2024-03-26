// For types that implement the Copy trait, like i32, the values are copied into the hash map.
// For owned values like String, the values will be moved and the hash map will be the owner.

use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();

    let old_value1 = map.insert(1, -1);
    match old_value1 {
        Some(val) => println!("Old value: {}", val),
        None => println!("Key was not present before!"),
    }

    let old_value2 = map.insert(1, 2);
    match old_value2 {
        Some(val) => println!("Old value: {}", val),
        None => println!("Key was not present before!"),
    }

    map.insert(5, 10);

    println!("{:?}", map);

    println!("Contains 5?: {}", map.contains_key(&5));
    println!("Contains 4?: {}", map.contains_key(&4));

    let removed_value = map.remove(&5);

    match removed_value {
        Some(val) => println!("Removed value: {}", val),
        None => println!("Key was not present before!"),
    }

    for k in map.keys() {
        println!("key: {k}");
    }

    for v in map.values() {
        println!("val: {v}");
    }

    let search_key = 1;
    let got_value = map.get(&search_key).copied().unwrap_or(0);
    println!("got_value: {}", got_value);
}
