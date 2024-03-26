use std::ops::Add;

#[derive(Debug)]
pub struct Point2D {
    x: f32,
    y: f32,
}

impl Add for Point2D {
    type Output = Point2D;

    fn add(self, other: Point2D) -> Point2D {
        Point2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add<&Point2D> for &Point2D {
    type Output = Point2D;

    fn add(self, other: &Point2D) -> Point2D {
        Point2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    let p1 = Point2D { x: 10.0, y: 2.0 };
    let p2 = Point2D { x: 3.4, y: -4.0 };

    // println!("{:?}", p1 + p2);
    // println!("{:?}", p1);
    // println!("{:?}", p2);

    println!("{:?}", (&p1) + (&p2));
    println!("{:?}", p1);
    println!("{:?}", p2);
}
