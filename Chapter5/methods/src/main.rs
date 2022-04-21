struct Point2D {
    x: f32,
    y: f32,
}

impl Point2D {
    fn origin() -> Point2D {
        Point2D { x: 0.0, y: 0.0 }
    }

    fn new(x: f32, y: f32) -> Point2D {
        Point2D { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point2D,
    p2: Point2D,
}

impl Rectangle {
    fn area(&self) -> f32 {
        let Point2D { x: x1, y: y1 } = self.p1;
        let Point2D { x: x2, y: y2 } = self.p2;

        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f32 {
        let Point2D { x: x1, y: y1 } = self.p1;
        let Point2D { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    fn translate(&mut self, x: f32, y: f32) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

fn main() {
    let mut rectangle = Rectangle {
        p1: Point2D::origin(),
        p2: Point2D::new(3.0, 4.0),
    };

    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point2D::origin(),
        p2: Point2D::new(1.0, 1.0),
    };

    rectangle.translate(1.0, 0.0);
    square.translate(1.0, 1.0);
}
