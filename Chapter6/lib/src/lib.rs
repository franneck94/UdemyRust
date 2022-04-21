pub fn public_function() {
    println!("called`public_function()`");
}

fn private_function() {
    println!("called`private_function()`");
}

pub fn indirect_access() {
    print!("called`indirect_access()`, that\n> ");

    private_function();
}
