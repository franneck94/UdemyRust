use std::fmt::Debug;
use std::fmt::Display;

trait HasArea {
    fn area(&self) -> f64;
}

#[derive(Debug)]
struct Rectangle {
    length: f64,
    height: f64,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Triangle {
    length: f64,
    height: f64,
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.length * self.height
    }
}

impl HasArea for Triangle {
    fn area(&self) -> f64 {
        (self.length * 0.5) * self.height
    }
}

fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}

fn area<T: HasArea>(t: &T) -> f64 {
    t.area()
}

fn main() {
    let rectangle = Rectangle {
        length: 3.0,
        height: 4.0,
    };
    let triangle = Triangle {
        length: 3.0,
        height: 4.0,
    };

    print_debug(&rectangle);
    println!("Area: {}", rectangle.area());

    print_debug(&triangle);
    println!("Area: {}", triangle.area());

    println!("Area: {}", area(&rectangle));
    println!("Area: {}", area(&triangle));
}
