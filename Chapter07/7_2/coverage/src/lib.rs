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

// https://github.com/xd009642/tarpaulin
// cargo install cargo-tarpaulin
// cargo tarpaulin --out html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_circumference() {
        let c1 = Circle { radius: 1.0 };

        assert_eq!(c1.compute_circumference(), 2.0 * 3.14159265359);
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
