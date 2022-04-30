#[derive(Debug)]
enum LoginData {
    None,
    Invalid,
    Not_Registered,
    Username(String),
}

fn main() {
    let none_user = LoginData::None;
    println!("{:?}", none_user);

    let admin = LoginData::Username(String::from("franneck94"));
    println!("{:?}", admin);
}
