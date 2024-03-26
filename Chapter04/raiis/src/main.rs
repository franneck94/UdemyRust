// RAII (Ressource Acquisition Is Initialization)

struct Data {
    name: String,
}

impl Drop for Data {
    fn drop(&mut self) {
        println!("Drop on {} is being called...", self.name);
    }
}

fn main() {
    {
        let mut _b1 = Box::new(5);
    }

    let mut _b2 = Box::new(Data {
        name: String::from("0xAA"),
    });

    {
        let _b3 = Box::new(Data {
            name: String::from("0xBB"),
        });

        _b2 = _b3; // Move
    }
}
