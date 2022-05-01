#[derive(Debug)]
pub struct StatisticData {
    data: Vec<f32>,
    mean: f32,
}

impl StatisticData {
    fn new() -> Self {
        Self {
            data: vec![],
            mean: 0.0_f32,
        }
    }

    pub fn append(&mut self, val: f32) {
        self.data.push(val);
    }

    pub fn remove(&mut self) -> Option<f32> {
        let popped_val = self.data.pop();

        popped_val
    }

    pub fn compute_mean(&mut self) {
        self.mean = self.data.iter().sum::<f32>() / (self.data.len() as f32);
    }
}

fn main() {
    let mut data1 = StatisticData {
        data: vec![],
        mean: 0.0,
    };

    let mut data2 = StatisticData::new();

    data2.append(1.0);
    data2.append(-1.0);
    data2.append(-1.0);

    data2.compute_mean();

    println!("{:#?}", data2);
}
