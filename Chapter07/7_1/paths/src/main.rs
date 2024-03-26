// An absolute path is the full path starting from a crate root;
// for code from an external crate, the absolute path begins with the crate name
// and for code from the current crate, it starts with the literal crate.

// A relative path starts from the current module and uses
// self, super, or an identifier in the current module.

// Using super allows us to reference an item that we know is in the parent module,
// it is like starting a filesystem path with the .. syntax.

fn func() {
    println!("called crate::func()");
}

mod foo {
    pub fn func2() {
        println!("called crate::foo::func2()");
        super::func();
    }

    pub fn func3() {
        println!("called crate::foo::func3()");
        self::func2();
    }
}

fn main() {
    crate::func(); // abs path
    crate::foo::func2(); // abs path
}
