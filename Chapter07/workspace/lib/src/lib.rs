pub fn adder(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(adder(2), 3);
        assert_eq!(adder(-2), -1);
    }
}
