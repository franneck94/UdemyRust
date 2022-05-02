#[derive(Debug)]
pub struct SplittedStr<'a> {
    remainder: &'a str,
    delimiter: &'a str,
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
    use super::*;

    #[test]
    fn it_works() {
        let haystack = "a b c d e";
        let letters = SplittedStr {
            remainder: haystack,
            delimiter: " ",
        };

        assert_eq!(
            letters.collect::<Vec<&str>>(),
            vec!["a", "b", "c", "d", "e"]
        );
    }
}
