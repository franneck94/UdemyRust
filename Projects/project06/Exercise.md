# Extended String Splitting Iterator Exercise with Unit Tests

## Objective

Extend the custom string-splitting iterator with unit tests to ensure its correctness and robustness.  
This exercise aims to reinforce understanding of Rust's testing framework and iterator behavior.

## Instructions

### Define Struct

The SplittedStr struct from the previous exercise is already defined.  
It contains fields for the remaining part of the string (remainder) and the delimiter to split by (delimiter).

### Write Unit Tests

Inside the tests module, write unit tests to validate the behavior of the SplittedStr iterator in different scenarios.  
Test cases should cover both empty and non-empty remainder strings, as well as empty and non-empty delimiters.  
Use assertions to verify the correctness of the iterator's output.

### Run Tests

Execute the tests using the cargo test command to ensure that the iterator functions as expected in all tested scenarios.  
Verify that all tests pass without errors.

#### Example Usage

```shell
test teststest_empty_remainder_empty_delimiter ... ok
test teststest_empty_remainder_non_empty_delimiter ... ok
test teststest_non_empty_remainder_empty_delimiter ... ok
test teststest_non_empty_remainder_non_empty_delimiter ... ok
test result ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Happy coding!
