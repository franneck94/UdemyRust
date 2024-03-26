#[derive(Debug)]
struct User {
    is_admin: bool,
    username: String,
    password: String,
}

impl User {
    fn build_admin(username: String, password: String) -> User {
        User {
            is_admin: true,
            username,
            password,
        }
    }
}

fn main() {
    let user1 = User {
        is_admin: true,
        username: String::from("Jan"),
        password: String::from("SubmarineFighter3"),
    };

    println!("{user1:?}");

    println!("{}", user1.is_admin);
    println!("{}", user1.username);
    println!("{}", user1.password);

    let mut user2 = User::build_admin(String::from("Peter"), String::from("P4ASSWORD"));

    println!("{user2:?}");

    println!("{}", user2.is_admin);
    println!("{}", user2.username);
    println!("{}", user2.password);

    user2.password = String::from("NewPassword");

    println!("{user2:?}");
}
