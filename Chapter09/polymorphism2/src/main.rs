pub trait Area {
    fn area(&self) -> f32;
}

#[derive(Debug)]
pub struct Circle {
    pub radius: f32,
}

impl Area for Circle {
    fn area(&self) -> f32 {
        self.radius * self.radius * std::f32::consts::PI
    }
}

#[derive(Debug)]
pub struct Square {
    length: f32,
}

impl Area for Square {
    fn area(&self) -> f32 {
        self.length * self.length
    }
}

// dynamic dispath
// fat pointer: points to data and virtual function table
// slower at runtime, smaller exe
fn example_to_call_area(obj: &dyn Area) -> f32 {
    obj.area()
}

fn main() {
    let circle = Circle { radius: 1.0 };
    let square = Square { length: 2.0 };

    println!("{}", example_to_call_area(&circle));
    println!("{}", example_to_call_area(&square));
}
