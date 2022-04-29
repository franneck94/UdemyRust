#[derive(Debug)]
pub struct SplittedStr<'a> {
    remainder: &'a str,
    delimiter: &'a str,
}

impl<'a> SplittedStr<'a> {
    pub fn new(haystack: &'a str, delimiter: &'a str) -> Self {
        Self {
            remainder: haystack,
            delimiter,
        }
    }
}

impl<'a> Iterator for SplittedStr<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next_delimiter) = self.remainder.find(self.delimiter) {
            let until = &self.remainder[..next_delimiter];
            self.remainder = &self.remainder[next_delimiter + self.delimiter.len()..];
            Some(until)
        } else if self.remainder.is_empty() {
            None
        } else {
            let rest = self.remainder;
            self.remainder = "";
            Some(rest)
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let haystack = "a b c d e";
        let letters: Vec<_> = super::SplittedStr::new(haystack, " ").collect();

        assert_eq!(letters, vec!["a", "b", "c", "d", "e"]);
    }
}
