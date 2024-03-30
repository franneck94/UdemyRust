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

fn main() {
    let circle = Box::new(Circle { radius: 1.0 });
    let square = Box::new(Square { length: 2.0 });

    // trait Objects via dnymaic dispatch
    // trait object fat pointer holds:
    // - data pointer
    // - vtable pointer
    let area_items: Vec<Box<dyn Area>> = vec![circle, square];
    for area_item in area_items {
        area_item.area();
    }
}
