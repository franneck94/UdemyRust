use std::collections::HashSet;

fn main() {
    let mut set = HashSet::new();

    set.insert(1);
    set.insert(2);
    let inserted = set.insert(2);

    println!("set {:?}", set);
    println!("inserted {:?}", inserted);

    for x in set.iter() {
        println!("Iter: {}", x);
    }

    let mut set2 = HashSet::new();
    set2.insert(1);
    set2.insert(3);
    println!("set2 {:?}", set2);

    let intersection = &set & &set2;
    println!("intersection {:?}", intersection);

    let union = &set | &set2;
    println!("union {:?}", union);

    let diff = &set - &set2;
    println!("diff {:?}", diff);
}
