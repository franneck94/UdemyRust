pub struct AverageList {
    data: Vec<f32>,
    average: f32,
}

impl AverageList {
    fn new() -> AverageList {
        AverageList {
            data: vec![],
            average: 0.0,
        }
    }

    pub fn add(&mut self, val: f32) {
        self.data.push(val);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<f32> {
        let popped_val = self.data.pop();

        match popped_val {
            Some(val) => {
                self.update_average();
                Some(val)
            }
            None => None,
        }
    }

    fn update_average(&mut self) {
        self.average = self.data.iter().sum::<f32>() / (self.data.len() as f32);
    }

    pub fn average(&self) -> f32 {
        self.average
    }
}

fn main() {
    let mut l = AverageList::new();

    l.add(1.0);
    l.add(-1.0);

    println!("Average: {}", l.average());

    l.add(-2.0);
    l.add(3.0);
    println!("Average: {}", l.average());
}
