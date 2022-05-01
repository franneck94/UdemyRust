mod my_mod;

fn function() {
    my_mod::public_interface();
    my_mod::nested::public_interface();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        super::function();
    }
}
