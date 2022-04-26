pub fn indirect_access() {
    println!("called my_mod::nested::indirect_access(), that\n> ");
    private_function();
}

fn private_function() {
    println!("called `my_mod::nested::private_function()`");
}
