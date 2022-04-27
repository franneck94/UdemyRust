#[derive(Debug)]
struct User {
    is_admin: bool,
    username: String,
    password: String,
}

#[derive(Debug)]
struct ExtendedUser {
    user: User,
    id: u32,
}

fn main() {
    let user1 = User {
        is_admin: true,
        username: String::from("Jan"),
        password: String::from("Ubootkämpfer3"),
    };
    println!("{}", user1.is_admin);
    println!("{}", user1.username);
    println!("{}", user1.password);
    println!("{:?}", user1);

    let user2 = build_admin(String::from("Peter"), String::from("P4SSW0RD"));
    println!("{}", user2.is_admin);
    println!("{}", user2.username);
    println!("{}", user2.password);

    let _user3 = ExtendedUser {
        user: user1,
        id: 1337_u32,
    };
    println!("{:?}", _user3);
}

fn build_admin(username: String, password: String) -> User {
    User {
        is_admin: true,
        username,
        password,
    }
}
