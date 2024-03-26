fn func1() {
    println!("called foo::func1()");
}

pub fn func2() {
    println!("called foo::func2()");
}

pub mod bar {
    fn func3() {
        println!("called bar::foo::func3()");
    }

    pub fn func4() {
        println!("called bar::foo::func4()");
        func3();
    }
}
