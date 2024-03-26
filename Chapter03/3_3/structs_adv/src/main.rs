#[derive(Debug)]
struct User {
    is_admin: bool,
    username: String,
    password: String,
}

#[derive(Debug)]
struct RGB(i32, i32, i32);

#[derive(Debug)]
struct Coord3D(i32, i32, i32);

fn main() {
    let user1 = User {
        is_admin: true,
        username: String::from("Jan"),
        password: String::from("SubmarineFighter3"),
    };

    println!("{user1:?}");

    // ..user1 must come last to specify that any remaining fields should get their values from user1
    let user2 = User {
        username: String::from("Bob"),
        ..user1
    };

    println!("{user2:?}");

    //  Each struct you define is its own type, even though the fields have the same types
    let black = RGB(0, 0, 0);
    let white = RGB(255, 255, 255);
    let origin = Coord3D(0, 0, 0);

    println!("{black:?}");
    println!("{white:?}");
    println!("{origin:?}");
}
