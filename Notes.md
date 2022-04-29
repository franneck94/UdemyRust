# Notes

## Key Features

- No Race Conditions
- No Exceptions
- No Memory Leaks

## Tools

Implemented features:

- Build System
- Package Manager
- Compiler
- Unit Testing
- Documentation Generator

Cargo:

- cargo new NAME
- cargo build
- cargo run
  - Would also run *cargo build* if binary is not up-to-date
- cargo install XYZ

## Memory

- Per default every variable is immutable

- Each value in rust is owned by one variable
- Whenever the owener goes out of scrope, the value will be deallocated
- There can only be ONE owner at a time

- There are only two options for passing variables of non-trivial types to functions
  - Move (standard case)
  - Reference (&dtype)
- There is no copy by value for non-trivial types!

- Borrowing in a scope
  - As many immutable references as we want, but then no mutable reference
  - Only one mutable reference, but then no immutable references

### Constants

Constants live for the entire lifetime of a program.  
Constants in Rust have no fixed address in memory, they’re effectively inlined to each place that they’re used.  

### Statics

Rust provides a ‘global variable’ sort of facility in static items.  
There is only one instance for each value, and it’s at a fixed location in memory.

## String

- String - like std::string in C++
- String literal - "blabla" - are immutable and has fixed size
- &str: String slice - like std::string_view in C++
  - String slice is borrowing!
- String growth/shrink as it is in C++
- String indexing is byte and not char based!

## Modules

- Everything in a module is private by default
- files in a sub-dir are like defining Modules with the mod keyword

## Result

- unwrap will panic (exit the programm) on failure
- match result:
  - Ok()
  - Err()

## Match

- *_* - like default case
- Combine arms with *|*

## Loops

- nested *loop* loops could have labels

## Array

- Fixed size known at compile time
- Size must not be fixed, when passing array as reference to a function
