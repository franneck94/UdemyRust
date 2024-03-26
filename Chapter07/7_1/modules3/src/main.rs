mod foo;

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
