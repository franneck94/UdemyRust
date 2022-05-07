use std::fmt;

type Result<T> = std::result::Result<T, NumberParseError>;

#[derive(Debug, Clone)]
struct NumberParseError;

impl fmt::Display for NumberParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid value")
    }
}

fn convert_first(vec: Vec<&str>) -> Result<f32> {
    vec.first().ok_or(NumberParseError).and_then(|s| {
        s.parse::<f32>()
            .map_err(|_| NumberParseError)
            .map(|i| 2.0 * i)
    })
}

fn print(result: Result<f32>) {
    match result {
        Ok(n) => println!("Ok: {}", n),
        Err(e) => println!("Err: {}", e),
    }
}

fn main() {
    let v1 = vec!["42"];
    let v2 = vec![" 42"];

    print(convert_first(v1));
    print(convert_first(v2));
}
