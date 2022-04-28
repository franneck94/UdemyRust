trait Living {
    fn get_name(&self) -> String {
        "no_name".to_owned()
    }

    fn get_age(&self) -> u32 {
        0
    }
}

struct Human {
    name: String,
    age: u32,
}

impl Human {
    fn new(&self, name: String, age: u32) -> Self {
        Human { name, age }
    }
}

impl Living for Human {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_age(&self) -> u32 {
        self.age
    }
}

fn main() {
    let name = "Jan";
    println!("Hello, {name}");
}
