pub mod nested;

fn private_interface() {
    println!("called private function");
}

pub fn public_interface() {
    private_interface();
}
