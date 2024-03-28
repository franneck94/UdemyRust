pub fn is_prime(n: u64) -> bool {
    if n == 0 || n == 1 {
        return false;
    }

    if (n == 2) || (n == 3) || (n == 5) || (n == 7) || (n == 11) || (n == 13) {
        return true;
    }

    if (n % 2 == 0)
        || (n % 3 == 0)
        || (n % 5 == 0)
        || (n % 7 == 0)
        || (n % 11 == 0)
        || (n % 13 == 0)
    {
        return false;
    }

    let upper = (n as f64).sqrt() as u64;

    (17..=upper).step_by(2).all(|i| n % i != 0)
}
