use regex::Regex;
use std::env;
use std::fs;
use text_colorizer::*;

#[derive(Debug)]
#[allow(dead_code)]
struct Arguments<'a> {
    pattern: &'a String,
    replace: &'a String,
    input_file: &'a String,
    output_file: &'a String,
}

fn print_help() {
    println!(
        "{} - replace a string with a new sring",
        "Find and replace".green()
    );
    println!(
        "{} <target string> <replacement string> <INPUT FILE> <OUTPUT FILE>",
        "Usage".green()
    );
}

fn replace(target: &str, rep: &str, data: &str) -> Result<String, regex::Error> {
    let regex = Regex::new(target)?;

    Ok(regex.replace_all(data, rep).to_string())
}

fn read(args: &Arguments) -> String {
    let data = match fs::read_to_string(&args.input_file) {
        Ok(f) => f,
        Err(e) => {
            eprintln!(
                "{} Failed to read from file {}: {:?}",
                "Error".red().bold(),
                args.input_file,
                e
            );
            std::process::exit(1);
        }
    };

    data
}

fn write(args: &Arguments, data: &String) {
    match fs::write(&args.output_file, &data) {
        Ok(_) => {}
        Err(e) => {
            eprintln!(
                "{} Failed to write to file {}: {:?}",
                "Error".red().bold(),
                args.output_file,
                e
            );
            std::process::exit(1);
        }
    }
}

fn parse_args<'a>(argc: usize, argv: &'a Vec<String>) -> Arguments<'a> {
    if argc == 2 && argv[1] == "--help" {
        print_help();
        std::process::exit(0);
    }

    if argc != 5 {
        eprintln!("{} wrong number of arguments", "ERROR".red());
        print_help();
        std::process::exit(1);
    }

    Arguments {
        pattern: &argv[1],
        replace: &argv[2],
        input_file: &argv[3],
        output_file: &argv[4],
    }
}

pub fn run(argc: usize, argv: Vec<String>) {
    let args = parse_args(argc, &argv);
    let data = read(&args);
    let replaced_data = match replace(&args.pattern, &args.replace, &data) {
        Ok(d) => d,
        Err(_) => {
            std::process::exit(1);
        }
    };
    write(&args, &replaced_data);
}

fn main() {
    let argv: Vec<String> = env::args().collect();
    let argc = argv.len();

    run(argc, argv);
}
