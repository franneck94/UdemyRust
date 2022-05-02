use std::cmp::Eq;
use std::cmp::PartialEq;
use std::ops::Add;

#[derive(Debug, Eq, PartialEq)]
pub struct Tensor1D<T>
where
    T: Add<T, Output = T>,
    T: Clone,
    T: Default,
{
    data: Vec<T>,
}

impl<T> Tensor1D<T>
where
    T: Add<T, Output = T>,
    T: Clone,
    T: Default,
{
    fn new() -> Self {
        Self { data: vec![] }
    }

    fn op(&self, rhs: &Self, op: fn(T, T) -> T) -> Self {
        if self.data.len() != rhs.data.len() {
            panic!("Unequal length!")
        }

        let mut result = Self {
            data: vec![T::default(); self.data.len()],
        };

        for idx in 0..result.data.len() {
            result.data[idx] = op(self.data[idx].clone(), rhs.data[idx].clone());
        }

        result
    }
}

impl<T> From<Vec<T>> for Tensor1D<T>
where
    T: Add<T, Output = T>,
    T: Clone,
    T: Default,
{
    fn from(d: Vec<T>) -> Self {
        Self { data: d }
    }
}

impl<T> Add for Tensor1D<T>
where
    T: Add<T, Output = T>,
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

impl<'a, T> Add<&'a Tensor1D<T>> for &'a Tensor1D<T>
where
    T: Add<T, Output = T>,
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

fn main() {
    // let v1: Tensor1D<f32> = Tensor1D::new();
    // println!("{:#?}", v1);
    // let v2: Tensor1D<f32> = Tensor1D::new();
    // println!("{:#?}", v2);

    // let v3 = v1 + v2;
    // println!("{:#?}", v3);

    let v1: Tensor1D<f32> = Tensor1D::from(vec![1.0, 2.0, 3.0]);
    println!("{:#?}", v1);
    let v2: Tensor1D<f32> = Tensor1D::from(vec![1.0, 2.0, 3.0]);
    println!("{:#?}", v2);

    let v3 = &v1 + &v2;
    println!("{:#?}", v1);
    println!("{:#?}", v2);
    println!("{:#?}", v3);

    println!("{:#?}", &v1 != &v2);
}
