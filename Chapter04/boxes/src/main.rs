use std::ops::Deref;

#[derive(Debug)]
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        println!("creating");
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("dropping");
    }
}

fn main() {
    // let b = Box::new(4);
    // println!("b = {}", b);
    // let c = &b;
    // println!("c = {}", c);

    let d = MyBox::new(4);
    println!("d = {:?}", d);
    let e = &d;
    println!("e = {:?}", e);
}
