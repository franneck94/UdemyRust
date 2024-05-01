# File Content Replacement Tool Exercise

## Objective

Implement a command-line tool that:

- reads a file's content
- performs a search-and-replace operation
- writes the modified content to an output file

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

The *Arguments* struct includes fields for the regular expression pattern, replacement string, input file path, and output file path.

### Argument Parsing

Implement the *parse_args* function to parse the user's command-line arguments and into an instance of the Arguments struct.  
Ensure that the correct number of arguments is provided, and display an error message if the number of arguments is incorrect.

### File Reading

Implement the read function to read the content of the input file specified by the user.  
Add error handling by displaying an error message if the reading fails.

### File Writing

Implement the write function to write the modified content to the output file specified by the user.  
Add error handling by displaying an error message if the writing fails.

### Search and Replace

Implement the replace function to perform a search-and-replace operation based on the provided regular expression pattern and replacement string.  
Use the regex crate for pattern matching and replacement.  
Ensure that errors are properly handled and propagated.

```rust
let regex = Regex::new(search_pattern)?;

Ok(regex.replace_all(data, replacement).to_string())
```

### Main Function

```rust
fn main() {
    let argv: Vec<String> = env::args().collect();
    let argc = argv.len();

    run(argc, argv) // Will call: parse_args, read_file, replace, write_file
}
```

Example Usage

```shell
cargo run "AA" "aa" input.txt output.txt
```

Happy coding!
