pub mod my_mod {
    fn private_interface() {
        println!("called private function");
    }

    pub fn public_interface() {
        private_interface();
    }

    pub mod my_mod_nested {
        pub fn public_interface() {
            println!("called nested function");
        }
    }
}

fn function() {
    my_mod::public_interface();
    my_mod::my_mod_nested::public_interface();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        super::function();
    }
}
