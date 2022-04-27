use std::collections::BinaryHeap;

fn main() {
    let mut bheap = BinaryHeap::new();

    bheap.push(1);
    bheap.push(5);
    bheap.push(2);
    bheap.push(3);
    bheap.push(4);

    println!("{:?}", bheap);

    bheap.pop();

    println!("{:?}", bheap);
    println!("{:?}", bheap.peek());
}
