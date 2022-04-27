#[derive(Debug)]
struct Circle {
    radius: f32,
}

impl Circle {
    fn compute_area(&self) -> f32 {
        self.radius * 3.14159265359
    }

    fn compute_circumference(&self) -> f32 {
        2.0 * self.radius * 3.14159265359
    }

    fn smaller(&self, other: &Self) -> bool {
        self.radius < other.radius
    }
}

fn main() {
    let c1 = Circle { radius: 1.0 };
    let c2 = Circle { radius: 2.0 };

    println!("Area: {}", c1.compute_area());
    println!("Circumference: {}", c1.compute_circumference());

    println!("c1 < c2: {}", c1.smaller(&c2));
}
