// A package is a bundle of one or more crates that provides a set of functionality.
// A package contains a Cargo.toml file that describes how to build those crates
// A package must contain at least one crate, whether that’s a library or binary crate.

fn private_interface() {
    println!("called function");
}

pub fn public_interface() {
    private_interface();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        super::public_interface();
        super::private_interface();
    }
}
