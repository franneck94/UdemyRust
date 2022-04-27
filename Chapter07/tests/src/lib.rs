#[derive(Debug)]
struct Rectangle {
    width: f32,
    height: f32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        if other.width < 0.0 || other.height < 0.0 {
            panic!("invalid data");
        }

        self.width >= other.width && self.height >= other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rectangle_can_hold() {
        let r1 = Rectangle {
            width: 5.0,
            height: 4.0,
        };
        let r2 = Rectangle {
            width: 4.5,
            height: 4.0,
        };

        assert_eq!(r1.can_hold(&r2), true);
        assert_ne!(r1.can_hold(&r2), false);

        assert!(r2.can_hold(&r2), "Failed massge {:?}", r2);
    }

    #[test]
    #[should_panic]
    fn rectangle_will_panic() {
        let r1 = Rectangle {
            width: 5.0,
            height: 4.0,
        };
        let r2 = Rectangle {
            width: -4.5,
            height: -4.0,
        };

        r1.can_hold(&r2);
    }
}
