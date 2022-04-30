// RAII (Ressource Acquisition Is Initialization)

struct Data;

impl Drop for Data {
    fn drop(&mut self) {
        println!("Drop is being called...");
    }
}

fn main() {
    let mut box1 = Box::new(5);

    let d = Box::new(Data {});
}
