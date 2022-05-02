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

impl<T> Tensor1D<T>
where
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Clone,
    T: Default,
{
    fn new() -> Self {
        Self { data: vec![] }
    }

    fn op(&self, rhs: &Self, op_func: fn(T, T) -> T) -> Self {
        let mut result = Self {
            data: vec![T::default(); self.data.len()],
        };

        for idx in 0..result.data.len() {
            result.data[idx] = op_func(self.data[idx].clone(), rhs.data[idx].clone());
        }

        result
    }
}

impl<T> From<Vec<T>> for Tensor1D<T>
where
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Clone,
    T: Default,
{
    fn from(data: Vec<T>) -> Self {
        Self { data }
    }
}

impl<T> Add for Tensor1D<T>
where
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Clone,
    T: Default,
{
    type Output = Tensor1D<T>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.data.len() != rhs.data.len() {
            panic!("Unequal length!")
        }

        self.op(&rhs, |v1, v2| v1 + v2)
    }
}

impl<T> Sub for Tensor1D<T>
where
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Clone,
    T: Default,
{
    type Output = Tensor1D<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.data.len() != rhs.data.len() {
            panic!("Unequal length!")
        }

        self.op(&rhs, |v1, v2| v1 - v2)
    }
}

fn main() {
    let _v: Tensor1D<f32> = Tensor1D::new();

    let v1 = Tensor1D::from(vec![1.0, 2.0, 3.0]);
    let v2 = Tensor1D::from(vec![1.0, 2.0, 3.0]);

    let v3 = v1 + v2;
    println!("v3 = {:#?}", v3);

    let v4 = Tensor1D::from(vec![1.0, 2.0, 3.0]);
    let v5 = v3 - v4;
    println!("v5 = {:#?}", v5);
}
