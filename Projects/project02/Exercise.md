# File Content Replacement Tool Exercise

## Objective

Implement a command-line tool that reads a file, performs a search-and-replace operation based on a regular expression pattern, and writes the modified content to another file.  
This exercise aims to familiarize students with command-line argument parsing, file I/O operations, regular expressions, and error handling in Rust.

## Instructions

### Command-Line Arguments

```rust
#[derive(Debug)]
struct Arguments<'a> {
    pattern: &'a String,
    replace: &'a String,
    input_file: &'a String,
    output_file: &'a String,
}
```

The Arguments struct includes fields for the regular expression pattern, replacement string, input file path, and output file path.

### Argument Parsing

Implement the parse_args function to parse command-line arguments and populate the Arguments struct.  
Ensure that the correct number of arguments is provided, and display an error message if the number of arguments is incorrect.

### File Reading

Implement the read function to read the content of the input file specified by the user.  
Handle errors gracefully by displaying an error message if reading fails.

### File Writing

Implement the write function to write the modified content to the output file specified by the user.  
Handle errors gracefully by displaying an error message if writing fails.

### Search and Replace

Implement the replace function to perform a search-and-replace operation based on the provided regular expression pattern and replacement string.  
Use the regex crate for pattern matching and replacement.  
Ensure that errors are properly handled and propagated.

```rust
let regex = Regex::new(search_pattern)?;

Ok(regex.replace_all(data, replacement).to_string())
```

### Main Function

Modify the main function to collect command-line arguments and pass them to the run function for processing: argument parsing, file reading, search-and-replace operation, and file writing.

Example Usage

```shell
cargo run "" "" input.txt output.txt
```

Happy coding!
