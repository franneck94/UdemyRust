use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();

    map.insert(1, 1);
    map.insert(5, 2);

    println!("{:?}", map);

    let old_value = map.insert(5, 10);

    println!("{:?}", old_value);
    println!("{:?}", map);

    println!("{:?}", map.contains_key(&5));

    let removed_value = map.remove(&5);

    println!("{:?}", removed_value);
    println!("{:?}", map);
}
