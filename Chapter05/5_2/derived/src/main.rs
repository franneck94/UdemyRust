#[derive(Debug)]
struct InnerData {
    value: i32,
    name: String,
}

#[derive(Debug)]
struct Data {
    inner: InnerData,
    idx: u64,
}

impl Data {
    fn print_me(&self) {
        println!("{}", self.idx);
    }
}

fn main() {
    let i = Data {
        idx: 1,
        inner: InnerData {
            value: 0,
            name: String::from("Jan"),
        },
    };

    println!("{:?}", i);
}
