mod splitter;

fn main() {
    let string = "a b c d e";
    let letters = splitter::SplittedStr {
        remainder: string,
        delimiter: " ",
    };

    for letter in letters {
        println!("{}", letter);
    }
}
