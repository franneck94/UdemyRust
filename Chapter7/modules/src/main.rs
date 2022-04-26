mod my_mod {
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

    pub mod nested {
        pub fn indirect_access() {
            println!("called my_mod::nested::indirect_access(), that\n> ");
            private_function();
        }

        fn private_function() {
            println!("called `my_mod::nested::private_function()`");
        }
    }
}

fn function() {
    println!("called `function()`");
}

// use crate::my_mod::nested::indirect_access;
// use crate::my_mod::nested::indirect_access as access;

fn main() {
    function();

    my_mod::function();

    my_mod::indirect_access();

    my_mod::nested::indirect_access();
    // indirect_access();
    // access();
}
