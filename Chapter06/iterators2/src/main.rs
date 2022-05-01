fn zipper() {
    let a1 = [1, 2, 3];
    let a2 = [-1, -2, -3];

    for (val1, val2) in a1.iter().zip(a2.iter()) {
        println!("{}, {}", val1, val2);
    }
}

fn enumerator() {
    let a = ['a', 'b', 'c'];

    for (idx, val) in a.iter().enumerate() {
        println!("a[{}] = {}", idx, val);
    }
}

fn for_eacher() {
    let a = ['a', 'b', 'c'];

    a.iter()
        .enumerate()
        .for_each(|(idx, val)| println!("a[{}] = {}", idx, val))
}

fn flattener() {
    let a = [['a', 'b', 'c'], ['d', 'e', 'f']];

    a.iter()
        .flatten()
        .enumerate()
        .for_each(|(idx, val)| println!("a[{}] = {}", idx, val))
}

fn main() {
    zipper();
    enumerator();
    for_eacher();
    flattener();
}
