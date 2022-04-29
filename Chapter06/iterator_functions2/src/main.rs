fn zipper() {
    let a1 = [1, 2, 3];
    let a2 = [4, 5, 6];

    let iter = a1.iter().zip(a2.iter());

    for (val1, val2) in iter {
        println!("{}, {}", val1, val2);
    }
}

fn enumerator() {
    let a = ['a', 'b', 'c'];

    let iter = a.iter().enumerate();

    for (idx, val) in iter {
        println!("{}, {}", idx, val);
    }
}

fn for_eacher() {
    let a = ['a', 'b', 'c'];

    a.iter()
        .enumerate()
        .for_each(|(idx, val)| println!("{}, {}", idx, val));
}

fn main() {
    // zipper();
    // enumerator();
    for_eacher();
}
