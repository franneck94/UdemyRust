use std::env;

mod find_and_replace;

fn main() {
    let argv: Vec<String> = env::args().collect();
    let argc = argv.len();

    find_and_replace::run(argc, argv);
}
