use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Data {
    value: i32,
}

fn main() {
    let mut set = HashSet::from([(Data { value: 2 }), (Data { value: 3 })]);

    println!("Contains 3?: {}", set.contains(&Data { value: 3 }));
    println!("Contains 4?: {}", set.contains(&Data { value: 4 }));

    set.insert(Data { value: 4 });
    println!("Contains 4?: {}", set.contains(&Data { value: 4 }));

    let removed = set.remove(&Data { value: 4 });

    let set1 = HashSet::from([1, 2]);
    let set2 = HashSet::from([2, 3]);

    let intersection = (&set1) & (&set2);
    let union = (&set1) | (&set2);
    let diff = (&set1) - (&set2);

    println!("intersection: {:?}", intersection);
    println!("union: {:?}", union);
    println!("diff: {:?}", diff);
}
