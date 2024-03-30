pub trait Area {
    fn area(&self) -> f32;
}

pub trait Circumference {
    fn circumference(&self) -> f32;
}

// Super Trait
pub trait GeoProperties: Area + Circumference {}

#[derive(Debug)]
pub struct Circle {
    pub radius: f32,
}

impl GeoProperties for Circle {}

impl Area for Circle {
    fn area(&self) -> f32 {
        self.radius * self.radius * std::f32::consts::PI
    }
}

impl Circumference for Circle {
    fn circumference(&self) -> f32 {
        2.0 * self.radius * std::f32::consts::PI
    }
}

#[derive(Debug)]
pub struct Square {
    length: f32,
}

impl GeoProperties for Square {}

impl Area for Square {
    fn area(&self) -> f32 {
        self.length * self.length
    }
}

impl Circumference for Square {
    fn circumference(&self) -> f32 {
        4.0 * self.length
    }
}

fn main() {
    let _circle = Box::new(Circle { radius: 1.0 });
    let _square = Box::new(Square { length: 2.0 });
}
