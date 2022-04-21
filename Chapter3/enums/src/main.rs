enum PermissionLevel {
    User,
    Instructor,
    Admin,
}

impl PermissionLevel {
    fn what_am_i(&self) -> &str {
        match self {
            PermissionLevel::User => "I am an User",
            PermissionLevel::Instructor => "I am an Instructor",
            PermissionLevel::Admin => "I am an Admin",
        }
    }
}

fn main() {
    let _a = PermissionLevel::User;
    let _b = PermissionLevel::Instructor;
    let _c = PermissionLevel::Admin;

    println!("{}", _a.what_am_i());

    // use crate::PermissionLevel::*;
    use crate::PermissionLevel::{Admin, Instructor};

    let _d = Admin;
    println!("{}", _d.what_am_i());
    let _e = Instructor;
    println!("{}", _e.what_am_i());
}
