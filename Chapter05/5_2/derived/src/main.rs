// Formatting: Display, Debug
// Comparison: PartialEq, Eq, PartialOrd, Ord
// Memory: Copy, Clone, Default
//  - Copy is implicit, inexpensive, and cannot be re-implemented (memcpy).
//  - Clone is explicit, may be expensive, and may be re-implemented arbitraril
// Algo: Hash

#[derive(Debug, Clone, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct InnerData {
    value: i32,
    name: String,
}

#[derive(Debug, Clone, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
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
