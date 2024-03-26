#[derive(Debug)]
pub struct SplittedStr<'a> {
    pub remainder: &'a str,
    pub delimiter: &'a str,
}

impl<'a> Iterator for SplittedStr<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next_delimiter_pos) = self.remainder.find(self.delimiter) {
            let remainder_until_delimiter = &self.remainder[..next_delimiter_pos];
            self.remainder = &self.remainder[next_delimiter_pos + self.delimiter.len()..];
            Some(remainder_until_delimiter)
        } else if self.remainder.is_empty() {
            None
        } else {
            let last_remainder = self.remainder;
            self.remainder = "";
            Some(last_remainder)
        }
    }
}
