#[derive(Debug)]
pub struct SplittedStr<'a> {
    pub remainder: &'a str,
    pub delimiter: &'a str,
}

impl<'a> Iterator for SplittedStr<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.delimiter.is_empty() && !self.remainder.is_empty() {
            let last_remainder = self.remainder;
            self.remainder = "";
            Some(last_remainder)
        } else if self.remainder.is_empty() {
            None
        } else if let Some(next_delimiter_pos) = self.remainder.find(self.delimiter) {
            let remainder_until_delimiter = &self.remainder[..next_delimiter_pos];
            self.remainder = &self.remainder[next_delimiter_pos + self.delimiter.len()..];
            Some(remainder_until_delimiter)
        } else {
            let last_remainder = self.remainder;
            self.remainder = "";
            Some(last_remainder)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_non_empty_remainder_non_empty_delimiter() {
        let string = "a b c";
        let letters = SplittedStr {
            remainder: string,
            delimiter: " ",
        };

        assert_eq!(letters.collect::<Vec<&str>>(), vec!["a", "b", "c"]);
    }

    #[test]
    fn test_empty_remainder_non_empty_delimiter() {
        let string = "";
        let letters = SplittedStr {
            remainder: string,
            delimiter: " ",
        };

        assert_eq!(letters.collect::<Vec<&str>>().len(), 0_usize);
    }

    #[test]
    fn test_empty_remainder_empty_delimiter() {
        let string = "";
        let letters = SplittedStr {
            remainder: string,
            delimiter: "",
        };

        assert_eq!(letters.collect::<Vec<&str>>().len(), 0_usize);
    }

    #[test]
    fn test_non_empty_remainder_empty_delimiter() {
        let string = "a b c";
        let letters = SplittedStr {
            remainder: string,
            delimiter: "",
        };

        assert_eq!(letters.collect::<Vec<&str>>(), vec!["a b c"]);
    }
}
