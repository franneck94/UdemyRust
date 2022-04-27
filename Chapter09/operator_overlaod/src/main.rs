use std::ops::Add;

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add<&Point> for &Point {
    type Output = Point;

    fn add(self, other: &Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 10.0, y: 2.0 };
    let p2 = Point { x: 3.4, y: -4.0 };

    // println!("{:?}", p1 + p2);
    // println!("{:?}", p1);
    // println!("{:?}", p2);

    println!("{:?}", &p1 + &p2);
    println!("{:?}", p1);
    println!("{:?}", p2);
}
