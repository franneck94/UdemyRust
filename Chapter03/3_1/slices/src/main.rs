use std::mem;

fn main() {
    let xs: [i32; 5] = [1, 2, 3, 4, 5]; // Index Range: 0, 1, 2, 3, 4

    println!("Length of xs: {}", xs.len());
    println!("Data Size of xs: {}", mem::size_of_val(&xs));

    let slice_xs = &xs[1..4]; // StartIndex(Included)..EndIndex(Excluded)

    println!("Slice of xs: {:?}", slice_xs); // Index Range: 1, 2, 3
    println!("Slice of xs: {:?}", &xs[1..=4]); // Index Range: 1, 2, 3, 4
    println!("Slice of xs: {:?}", &xs[1..5]); // Index Range: 1, 2, 3, 4
}
