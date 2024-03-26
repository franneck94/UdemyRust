#[derive(Debug)]
enum PermissionLevel {
    User,
    Instructor,
    Admin,
}

impl PermissionLevel {
    fn description(&self) -> String {
        match self {
            PermissionLevel::User => String::from("I am an User"),
            PermissionLevel::Instructor => String::from("I am an Instructor"),
            PermissionLevel::Admin => String::from("I am an Admin"),
        }
    }

    fn is_admin1(&self) -> bool {
        match self {
            PermissionLevel::Admin => true,
            _ => false,
        }
    }

    // The syntax if let takes a pattern and an expression separated by an equal sign.
    // if let is like a match that runs code when the value matches one pattern and ignores all other.
    fn is_admin2(&self) -> bool {
        if let PermissionLevel::Admin = self {
            true
        } else {
            false
        }
    }
}

fn main() {
    let user1 = PermissionLevel::Admin;
    println!("{:?}", user1);
    println!("{}", user1.description());
    println!("{}", user1.is_admin1());
    println!("{}\n", user1.is_admin2());

    let user2 = PermissionLevel::Instructor;
    println!("{:?}", user2);
    println!("{}", user2.description());
    println!("{}", user2.is_admin1());
    println!("{}\n", user2.is_admin2());

    let user3 = PermissionLevel::User;
    println!("{:?}", user3);
    println!("{}", user3.description());
    println!("{}", user3.is_admin1());
    println!("{}\n", user3.is_admin2());
}
