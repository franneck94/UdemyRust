use std::cmp::Eq;
use std::cmp::PartialEq;
use std::ops::Add;
use std::ops::Sub;

#[derive(Debug, Eq, PartialEq)]
struct Tensor1D<T>
where
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Clone,
    T: Default,
{
    data: Vec<T>,
}

fn main() {
    let _v: Tensor1D<f32> = Tensor1D::new();

    let v1 = Tensor1D::from(vec![1.0, 2.0, 3.0]);
    println!("v1 = {:#?}", v1);
    let v2 = Tensor1D::from(vec![1.0, 2.0, 3.0]);
    println!("v2 = {:#?}", v2);
    let v3 = v1 + v2;
    println!("v3 = {:#?}", v3);

    let v4 = Tensor1D::from(vec![1.0, 2.0, 3.0]);
    let v5 = v3 - v4;
    println!("v5 = {:#?}", v5);
}
