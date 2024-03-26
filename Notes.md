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
- Whenever the owner goes out of scope, the value will be deallocated
- There can only be ONE owner at a time
- There are only two options for passing variables of non-trivial types to functions
  - Move (standard case)
  - Reference (&dtype)
- There is no copy by value for non-trivial types!
- Borrowing in a scope
  - As many immutable references as we want, but then no mutable reference
  - Only one mutable reference, but then no immutable references

## Pointer

Rc<T>, the Reference Counted Smart Pointer  
When a single value might have multiple owners.  
For example, in graph data structures, multiple edges might point to the same node, and that node is conceptually owned by all of the edges that point to it.

## Constants

- Constants live for the entire lifetime of a program.  
- Constants in Rust have no fixed address in memory, they’re effectively inlined to each place that they’re used.  

## Statics

- Rust provides a ‘global variable’ sort of facility in static items.  
- There is only one instance for each value, and it’s at a fixed location in memory.

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

- unwrap will panic (exit the program) on failure
- match result:
  - Ok()
  - Err()

## Expect vs Unwrap

- expect == unwrap with a message
- expect_err == unwrap_err with a message

## Match

- *_* - like default case
- Combine arms with *|*

## Loops

- nested *loop* loops could have labels

## Iterators

- into_iter may yield any of T, &T or &mut T, depending on the context
- iter will yield &T, by convention
- iter_mut will yield &mut T, by convention

## Array

- Fixed size known at compile time
- Size must not be fixed, when passing array as reference to a function

## Arc

The type Arc<T> provides shared ownership of a value of type T, allocated in the heap.  
Invoking clone on Arc produces a new Arc instance, which points to the same allocation on the heap as the source Arc, while increasing a reference count.

## Closure Traits

Closures can capture values from their environment in three ways, which directly map to the three ways a function can take a parameter:

- taking ownership
- borrowing mutably
- and borrowing immutably

- *FnOnce* moves the variables it captures, can’t take ownership of the same variables more than once
- *FnMut* can change the environment because it mutably borrows values
- *Fn* borrows values from the environment immutably
