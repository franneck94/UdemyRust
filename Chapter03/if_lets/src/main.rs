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

    fn is_admin(&self) -> bool {
        let ret = if let PermissionLevel::Admin = self {
            true
        } else {
            false
        }

        ret
    }
}

fn main() {
    let user1 = PermissionLevel::Admin;
    println!("{:?}", user1);
    println!("{}", user1.is_admin());

    let user2 = PermissionLevel::Instructor;
    println!("{:?}", user2);
    println!("{}", user2.is_admin());

    let user3 = PermissionLevel::User;
    println!("{:?}", user3);
    println!("{}", user3.is_admin());
}
