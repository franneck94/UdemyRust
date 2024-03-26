// multiple ownership by using the type Rc<T>
// abbreviation for reference counting
// Rc<T> type keeps track of the number of references
// Cloning an Rc<T> Increases the Reference Count
use std::rc::Rc;

#[derive(Debug)]
struct Car {
    name: String,
}

impl Drop for Car {
    fn drop(&mut self) {
        println!("Called drop for vehicle: {}", self.name);
    }
}

fn box_example() {
    let car1 = Box::new(Car {
        name: String::from("Audi RS3"),
    });

    let car2 = car1; // Move!

    println!("{:#?}", car1);
    println!("{:#?}", car2);
}

fn rc_example() {
    let car1 = Rc::new(Car {
        name: String::from("Audi RS3"),
    });

    {
        let car2 = car1.clone();
        println!("{:#?}", car1);
        println!("{:#?}", car2);
    }

    println!("{:#?}", car1);
}

fn main() {
    // box_example();
    rc_example();
}
