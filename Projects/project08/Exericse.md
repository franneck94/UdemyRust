# Custom 1D Tensor Operations Exercise

## Objective

Create a generic 1D tensor structure capable of performing addition and subtraction operations on vectors of numeric types. This exercise aims to reinforce understanding of generics, trait bounds, operator overloading, and error handling in Rust.

## Instructions

### Define Tensor1D Struct

Create a Tensor1D struct parameterized by the type T, where T represents the numeric type of the tensor elements.  
Implement the Debug, Eq, and PartialEq traits for the struct.

### Implement Constructor and Operation Methods

Implement a new constructor method for creating an empty tensor. Implement an op method to perform element-wise operations (addition or subtraction) between two tensors of the same length.  
Use a closure as a parameter to specify the operation function (op_func).   
Ensure that the tensor data is cloned before performing the operation.

### Implement From Trait for Vec

Implement the From<Vec<T>> trait for converting a vector of elements into a Tensor1D.  
This allows for convenient initialization of tensors from vectors.

### Implement Add and Sub Traits

Implement the Add and Sub traits for adding and subtracting tensors, respectively.  
Ensure that tensors of unequal lengths cannot be added or subtracted by panicking with an error message.

### Main Function

In the main function, demonstrate the usage of the Tensor1D struct by creating instances of tensors from vectors and performing addition and subtraction operations between them.  
Print out the resulting tensors to verify correctness.

#### Example Output

```rust
v1 = Tensor1D { data [1.0, 2.0, 3.0] }
v2 = Tensor1D { data [1.0, 2.0, 3.0] }
v3 = Tensor1D { data [2.0, 4.0, 6.0] }
v5 = Tensor1D { data [1.0, 2.0, 3.0] }
```

Happy coding!
