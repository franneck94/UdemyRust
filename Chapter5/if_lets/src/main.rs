enum Foo {
    Bar,
    Baz,
}

fn main() {
    // Create example variables
    let a = Foo::Bar;
    let b = Foo::Baz;

    // Variable a matches Foo::Bar
    if let Foo::Bar = a {
        println!("a is foobar");
    }

    // Variable b does not match Foo::Bar
    // So this will print nothing
    if let Foo::Bar = b {
        println!("b is foobar");
    }
}
