#[derive(Debug)]
struct Circle {
    pub radius: f32,
}

impl Circle {
    fn compute_area(&self) -> f32 {
        self.radius * self.radius * 3.14159265359
    }

    fn compute_circumference(&self) -> f32 {
        2.0 * self.radius * 3.14159265359
    }

    fn smaller(&self, other: &Self) -> bool {
        if self.radius < 0.0 || other.radius < 0.0 {
            panic!("invalid data");
        }

        self.radius < other.radius
    }
}

// 1. Set up any needed data or state.
// 2. Run the code you want to test.
// 3. Assert the results are what you expect

// To run all tests: cargo test
// To run one test: cargo test EXACT_TEST_NAME
// To run tests regarding a pattern: cargo test PATTERN (e.g. cargo test test_circle_)

// by default they run in parallel using threads
// you must make sure your tests don’t depend on each other or on any shared state

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_circumference() {
        let c1 = Circle { radius: 1.0 }; // 1.

        let res = c1.compute_circumference(); // 2

        assert_eq!(res, 2.0 * 3.14159265359); // 3
    }

    #[test]
    fn test_circle_area() {
        let c1 = Circle { radius: 1.0 };

        assert_eq!(c1.compute_area(), 3.14159265359);
    }

    #[test]
    fn test_circle_smaller() {
        let c1 = Circle { radius: 1.0 };
        let c2 = Circle { radius: 2.0 };

        assert_eq!(c1.smaller(&c2), true);
        assert_eq!(c2.smaller(&c1), false);

        assert!(c1.smaller(&c2));
    }

    #[test]
    #[should_panic]
    fn test_panic() {
        let c1 = Circle { radius: -1.0 };
        let c2 = Circle { radius: 2.0 };

        c1.smaller(&c2);
    }
}
