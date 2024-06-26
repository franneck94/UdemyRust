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

// static dispatch
// create function copy at comple time, faster in runtime, larger exe size
fn example_to_call_area<T>(obj: &T) -> f32
where
    T: Area,
{
    obj.area()
}

fn main() {
    let circle = Circle { radius: 1.0 };
    let square = Square { length: 2.0 };

    println!("{}", example_to_call_area(&circle));
    println!("{}", example_to_call_area(&square));
}
