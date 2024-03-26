#[derive(Debug)]
struct Data {
    d: [i32; 3],
}

struct DataIter<'a> {
    data: &'a Data,
    index: usize,
}

impl<'a> std::iter::IntoIterator for &'a Data {
    type Item = &'a i32;
    type IntoIter = DataIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        DataIter {
            data: self,
            index: 0,
        }
    }
}

impl<'a> std::iter::Iterator for DataIter<'a> {
    type Item = &'a i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.data.d.len() {
            let item = &self.data.d[self.index];
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

fn main() {
    let data = Data { d: [1, 2, 3] };

    // Iterate over each element in Data
    for d in data.into_iter().take(2).skip(1) {
        println!("{}", d);
    }
}
