pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    components: Vec<Box<dyn Draw>>,
}

impl Screen {
    fn run(&self) {
        for comp in self.components.iter() {
            comp.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Drwaing button...");
    }
}

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("Drwaing button...");
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(Button {
                width: 100,
                height: 100,
                label: String::from("Button"),
            }),
            Box::new(SelectBox {
                width: 100,
                height: 100,
                options: vec![String::from("Option1"), String::from("Option2")],
            }),
        ],
    };

    screen.run();
}
