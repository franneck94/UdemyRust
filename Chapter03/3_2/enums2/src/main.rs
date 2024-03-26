#[derive(Debug)]
enum LoginData {
    None,
    Invalid,
    Not_Registered,
    Username(String),
}

fn main() {
    let none_user = LoginData::None;
    println!("{none_user:?}");

    let invalid_user = LoginData::Invalid;
    println!("{invalid_user:?}");

    let not_reg_user = LoginData::Not_Registered;
    println!("{not_reg_user:?}");

    let admin_user = LoginData::Username(String::from("franneck94"));
    println!("{admin_user:?}");

    match admin_user {
        LoginData::None | LoginData::Invalid | LoginData::Not_Registered => {
            println!("User Unknown")
        }
        LoginData::Username(_) => println!("{admin_user:?}"),
    }
}
