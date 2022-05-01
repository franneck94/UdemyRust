#[derive(Debug)]
struct Circle {
    radius: f32,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle() {
        let c1 = Circle { radius: 1.0 };
        let c2 = Circle { radius: 2.0 };

        assert_eq!(c1.smaller(&c2), true);
        assert_eq!(c2.smaller(&c1), false);

        assert_ne!(c1.smaller(&c2), false);
        assert_ne!(c2.smaller(&c1), true);
    }

    #[test]
    #[should_panic]
    fn test_panic() {
        let c1 = Circle { radius: -1.0 };
        let c2 = Circle { radius: 2.0 };

        c1.smaller(&c2);
    }
}
