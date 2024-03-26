//! Circle library.
//!
//! Common utility functions for circles.
//!
//! To generate the HTML run: cargo doc

/// This is a brief descr of the Circle sturct.
#[derive(Debug)]
pub struct Circle {
    pub radius: f32,
}

impl Circle {
    /// [short sentence explaining what it is]
    ///
    /// [more detailed explanation]
    ///
    /// [at least one code example that users can copy/paste to try it]
    ///
    /// [even more advanced explanations if necessary]
    pub fn compute_area(&self) -> f32 {
        self.radius * self.radius * 3.14159265359
    }

    /// Compute the circumference of a [Circle].
    ///
    /// Based on the radius of the self instance, the circumference is computed.
    ///
    /// ```rust
    /// c = Circle { radius: 1.0 };
    /// c.compute_circumference()
    /// ```
    ///
    /// The function cannot panic.
    pub fn compute_circumference(&self) -> f32 {
        2.0 * self.radius * 3.14159265359
    }

    /// Check if the circle is smaller than another circle.
    ///
    /// The circles are compared based on the radius of the self and the other instance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// c1 = Circle { radius: 1.0 };
    /// c2 = Circle { radius: 2.0 };
    /// c1.smaller(c2)
    /// ```
    ///
    /// # Panics
    ///
    /// The function panics if any of the radius is smaller than zero.
    pub fn smaller(&self, other: &Self) -> bool {
        if self.radius < 0.0 || other.radius < 0.0 {
            panic!("invalid data");
        }

        self.radius < other.radius
    }
}

// cargo doc

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
