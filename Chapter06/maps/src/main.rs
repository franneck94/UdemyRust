use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();

    let old_value1 = map.insert(1, -1);
    match old_value1 {
        Some(val) => println!("Old value: {}", val),
        None => println!("Key was not present"),
    }

    map.insert(5, 3);

    println!("{:?}", map);

    let old_value2 = map.insert(5, 10);

    match old_value2 {
        Some(val) => println!("Old value: {}", val),
        None => println!("Key was not present"),
    }

    println!("Contains 5?: {:?}", map.contains_key(&5));
    println!("Contains 4?: {:?}", map.contains_key(&4));

    let removed_value = map.remove(&5);

    match removed_value {
        Some(val) => println!("Removed val: {}", val),
        None => println!("Key was not present"),
    }

    for k in map.keys() {
        println!("Key: {}", k);
    }
}
