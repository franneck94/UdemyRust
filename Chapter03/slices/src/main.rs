use std::mem;

fn print_slice(slice: &[i32]) {
    println!("Slice: {:?}", &slice);
}

fn main() {
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    println!("length: {}", xs.len());
    println!("data size: {}", mem::size_of_val(&xs));

    println!("Slice: {:?}", &xs[1..4]);
    println!("Slice: {:?}", &xs[1..=3]);

    print_slice(&xs[1..3]);
}
