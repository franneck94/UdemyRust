fn is_prime(n: u64) -> bool {
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

    let upper_value = (n as f64).sqrt() as u64;

    (17..=upper_value).step_by(2).all(|i| n % i != 0)
}

fn main() {
    let number = 1237;
    let number_is_prime = is_prime(number);
    println!("{} is prime {}", number, number_is_prime);

    let number = 1235;
    let number_is_prime = is_prime(number);
    println!("{} is prime {}", number, number_is_prime);
}
