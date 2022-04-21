pub mod nested;

fn private_function() {
    println!("called `my_mod::private_function()`");
}

pub fn function() {
    println!("called `my_mod::function()`");
}

pub fn indirect_access() {
    print!("called `my_mod::indirect_access()`, that\n> ");
    private_function();
}
