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
    }
}
