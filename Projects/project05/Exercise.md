# Custom String Splitting Iterator Exercise

## Objective

Implement a custom iterator for splitting a string into parts based on a delimiter.  
This exercise aims to deepen understanding of Rust's iterator trait implementation and string manipulation techniques.

## Instructions

### Define Struct

Create a SplittedStr struct to represent the iterator.  
his struct should contain fields for the remaining part of the string (remainder) and the delimiter to split by (delimiter).  
Ensure that the lifetimes of the fields are appropriately annotated.

### Implement Iterator Trait

Implement the Iterator trait for the SplittedStr struct.  
Define the associated type Item as a reference to a string slice (&str).  
Implement the next method to split the remainder string into parts based on the delimiter.  
Handle cases where the delimiter is found and where the end of the string is reached.

### Use Case

In the main function or in a separate test function, demonstrate the usage of the SplittedStr iterator by splitting a string into parts using a specified delimiter.  
Print or display the results to verify correctness.

#### Example Input

```rust
let input_string = "apple,banana,orange";
let delimiter = ",";

let splitted_str = SplittedStr {
    remainder input_string,
    delimiter delimiter,
};

for part in splitted_str {
    println!("{}", part);
}
```

#### Example Output

```rust
apple
banana
orange
```

Happy coding!
