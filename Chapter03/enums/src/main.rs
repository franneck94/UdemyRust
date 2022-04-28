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

    let _d = PermissionLevel::Admin;
    println!("{}", _d.what_am_i());
    let _e = PermissionLevel::Instructor;
    println!("{}", _e.what_am_i());
}
