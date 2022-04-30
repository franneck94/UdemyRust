#[derive(Debug)]
struct User {
    is_admin: bool,
    username: String,
    password: String,
}

fn build_admin(username: String, password: String) -> User {
    User {
        is_admin: true,
        username,
        password,
    }
}

fn main() {
    let user1 = User {
        is_admin: true,
        username: String::from("Jan"),
        password: String::from("Ubootkämpfer3"),
    };

    println!("{:?}", user1);
    println!("{}", user1.is_admin);
    println!("{}", user1.username);
    println!("{}", user1.password);

    let user2 = build_admin(String::from("Peter"), String::from("P4SSW0RD"));

    println!("{:?}", user2);
    println!("{}", user2.is_admin);
    println!("{}", user2.username);
    println!("{}", user2.password);
}
