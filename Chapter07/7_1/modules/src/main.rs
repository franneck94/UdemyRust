// Declaring modules: The compiler will look for "foo" in these places:
// Inline, within curly brackets that replace the semicolon following "mod foo"
// In the file src/foo.rs
// In the file src/foo/mod.rs

// Declaring submodules: The compiler will look for "bar" in these places:
// Inline, directly following mod foo, within curly brackets instead of the semicolon
// In the file src/foo/bar.rs
// In the file src/foo/bar/mod.rs

// use keyword creates shortcuts to items to reduce repetition of long paths

mod foo {
    fn func1() {
        println!("called foo::func1()");
    }

    pub fn func2() {
        println!("called foo::func2()");
    }

    pub mod bar {
        fn func3() {
            println!("called foo::bar::func3()");
        }

        pub fn func4() {
            println!("called foo::bar::func4()");
            func3();
        }
    }
}

fn func() {
    println!("called func()");
}

fn main() {
    func();

    // foo::func1(); // Error

    foo::func2();

    // foo::bar::func3(); // Error

    foo::bar::func4();
}
