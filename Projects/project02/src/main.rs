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

fn read(args: &Arguments) -> String {
    let data = match fs::read_to_string(&args.input_file) {
        Ok(f) => f,
        Err(_) => {
            eprintln!(
                "{} Failed to read from file {}!",
                "ERROR".red(),
                &args.input_file
            );
            std::process::exit(1);
        }
    };

    data
}

fn replace(search_target: &str, replacement: &str, data: &str) -> Result<String, regex::Error> {
    let regex = Regex::new(search_target)?;

    Ok(regex.replace_all(data, replacement).to_string())
}

fn write(args: &Arguments, data: &String) {
    match fs::write(&args.output_file, &data) {
        Ok(_) => {}
        Err(_) => {
            eprintln!(
                "{} Failed to write to file {}!",
                "ERROR".red(),
                &args.input_file
            );
            std::process::exit(1);
        }
    }
}

fn run(argc: usize, argv: Vec<String>) {
    let args = parse_args(argc, &argv);
    let data = read(&args);
    let replaced_data = match replace(&args.pattern, &args.replace, &data) {
        Ok(d) => d,
        Err(_) => std::process::exit(1),
    };
    write(&args, &replaced_data);
}

fn main() {
    let argv: Vec<String> = env::args().collect();
    let argc = argv.len();

    run(argc, argv);
}
