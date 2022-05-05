use std::ops::Deref;
use std::ops::DerefMut;

pub struct MyBox<T> {
    value: T,
}

impl<T> MyBox<T> {
    fn new(x: T) -> Self {
        Self { value: x }
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("Dropping MyBox");
    }
}

fn main() {
    let a = 1;
    let b = &a;

    assert_eq!(a, 1);
    assert_eq!(*b, 1);

    let c = Box::new(a);

    assert_eq!(*c, 1);

    let mut d = MyBox::new(a);

    assert_eq!(*d, 1);

    *d = 2;

    assert_eq!(*d, 2);
}
