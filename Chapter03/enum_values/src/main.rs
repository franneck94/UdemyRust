#[derive(Debug)]
enum IpAddressKind {
    V4(String),
    V6(String),
}

fn main() {
    let localhost = IpAddressKind::V4(String::from("192.168.0.1"));
    println!("{:#?}", localhost);

    let localhost2 = IpAddressKind::V6(String::from("FFAA.ABCC.0000.0001"));
    println!("{:#?}", localhost2);
}
