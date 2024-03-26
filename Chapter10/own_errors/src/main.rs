use std::fmt;

#[derive(Debug, Clone)]
struct NumberParseError;

type ParsingResult<T> = std::result::Result<T, NumberParseError>;

impl fmt::Display for NumberParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "error")
    }
}

fn convert_i32(input: &str) -> ParsingResult<i32> {
    input.parse::<i32>().map_err(|_| NumberParseError)
}

fn convert_f32(input: &str) -> ParsingResult<f32> {
    input.parse::<f32>().map_err(|_| NumberParseError)
}

fn main() {
    let v = " 2";

    let result = convert_i32(v);

    match result {
        Ok(result) => println!("Result: {result}"),
        Err(err) => println!("Err: {err}"),
    }
}
