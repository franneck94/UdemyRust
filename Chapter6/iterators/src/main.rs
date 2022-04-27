use std::collections::HashSet;

fn main() {
    let v1 = vec![1, 2, 3];
    let iterator = v1.iter();

    for val in iterator {
        println!("{}", val);
    }

    let mut v2 = vec![1, 2, 3];
    let iterator2 = v2.iter_mut();

    for val in iterator2 {
        *val *= 2;
    }

    let iterator2 = v2.iter_mut();
    for val in iterator2 {
        println!("{}", val);
    }

    let v3: HashSet<i32> = v2.iter().map(|val| val / 2).collect();
    println!("{:#?}", v3);
}
