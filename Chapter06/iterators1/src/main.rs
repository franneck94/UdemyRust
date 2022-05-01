fn mapping(data: &mut Vec<String>) -> Vec<Vec<&str>> {
    data.iter().map(|val| val.split(' ').collect()).collect()
}

fn filtering(data: &mut Vec<String>) -> Vec<&String> {
    data.iter()
        .filter(|val| val.parse().unwrap_or(0) > 200)
        .collect()
}

fn iterator_stepping(data: &mut Vec<i32>) {
    for d in data.iter().skip(2).take(4) {
        println!("{}", d);
    }
}

fn main() {
    let mut data = vec![
        String::from("1 2 3"),
        String::from("4 5 6"),
        String::from("7 8 9"),
    ];
    println!("{:?}", data);

    let mapped_data = mapping(&mut data);
    println!("{:?}", mapped_data);

    let mut data2 = vec![
        String::from("123"),
        String::from("456"),
        String::from("789"),
    ];
    let filtered_data = filtering(&mut data2);
    println!("{:?}", filtered_data);

    let mut data3 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    iterator_stepping(&mut data3);
}
