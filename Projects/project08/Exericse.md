# Custom 1D Tensor Operations Exercise

## Objective

Create a generic 1D tensor structure capable of performing addition and subtraction operations on vectors of numeric types.  
This exercise aims to reinforce understanding of generics, trait bounds, operator overloading, and error handling in Rust.

## Instructions

### Define Tensor1D Struct

Create a Tensor1D struct parameterized by the type T, where T represents the numeric type of the tensor elements.  
Implement the Debug, Eq, and PartialEq traits for the struct.

### Implement Constructor and Operation Methods

Implement a new constructor method for creating an empty tensor. Implement an op method to perform element-wise operations (addition or subtraction) between two tensors of the same length.  
Use a closure as a parameter to specify the operation function (op_func).  
Ensure that the tensor data is cloned before operating.

### Implement From Trait for Vec

Implement the *From<Vec\<T\>>* trait for converting a vector of elements into a Tensor1D.  
This allows for the convenient initialization of tensors from vectors.

### Implement Add and Sub Traits

Implement the Add and Sub traits for adding and subtracting tensors, respectively.  
Ensure that tensors of unequal lengths cannot be added or subtracted by panicking with an error message.

### Main Function

```rust
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
```

Happy coding!
