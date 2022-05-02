#[derive(Debug)]
struct SplittedStr<'a> {
    string: &'a str,
    delimiter: &'a str,
}

impl<'a> Iterator for SplittedStr<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(pos_delimiter) = self.string.find(self.delimiter) {
            let slice = &self.string[..pos_delimiter];
            self.string = &self.string[pos_delimiter + self.delimiter.len()..];
            Some(slice)
        } else if self.string.is_empty() {
            None
        } else {
            let last_slice = self.string;
            self.string = "";
            Some(last_slice)
        }
    }
}

fn main() {
    let string = "a b c d e";
    let letters = SplittedStr {
        string,
        delimiter: " ",
    };

    for letter in letters {
        println!("{}", letter);
    }
}
