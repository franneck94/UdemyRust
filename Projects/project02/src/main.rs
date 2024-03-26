use colored::*;
use regex::Regex;
use std::env;
use std::fs;

#[derive(Debug)]
struct Arguments<'a> {
    pattern: &'a String,
    replace: &'a String,
    input_file: &'a String,
    output_file: &'a String,
}

fn parse_args(argc: usize, argv: &Vec<String>) -> Arguments {
    if argc != 5 {
        eprintln!("{} wrong number of arguments!", "ERROR".red());
        std::process::exit(1);
    }

    Arguments {
        pattern: &argv[1],
        replace: &argv[2],
        input_file: &argv[3],
        output_file: &argv[4],
    }
}

fn read(filepath: &str) -> String {
    match fs::read_to_string(filepath) {
        Ok(data) => data,
        Err(_) => {
            eprintln!("{} Failed to read from file {}!", "ERROR".red(), &filepath);
            std::process::exit(1);
        }
    }
}

fn write(output_file: &str, data: &str) {
    match fs::write(&output_file, &data) {
        Ok(_) => {}
        Err(_) => {
            eprintln!(
                "{} Failed to write to file {}!",
                "ERROR".red(),
                &output_file
            );
            std::process::exit(1);
        }
    }
}

fn replace(search_pattern: &str, replacement: &str, data: &str) -> Result<String, regex::Error> {
    let regex = Regex::new(search_pattern)?;

    Ok(regex.replace_all(data, replacement).to_string())
}

fn run(argc: usize, argv: Vec<String>) {
    let args = parse_args(argc, &argv);
    let data = read(&args.input_file);

    let replaced_data = match replace(&args.pattern, &args.replace, &data) {
        Ok(data) => data,
        Err(_) => {
            eprintln!("{} Failed to replace data!", "ERROR".red(),);
            std::process::exit(1);
        }
    };

    write(&args.output_file, &replaced_data);
}

fn main() {
    // 1. regexp pattern
    // 2. replace
    // 3. Input File
    // 4. Output File
    // cargo run "" "" C:\Users\Jan\OneDrive\_Coding\UdemyRust\Projects\project02\Cargo.toml C:\Users\Jan\OneDrive\_Coding\UdemyRust\Projects\project02\CargoNEW.toml

    let argv: Vec<String> = env::args().collect();
    let argc = argv.len();

    run(argc, argv)
}
