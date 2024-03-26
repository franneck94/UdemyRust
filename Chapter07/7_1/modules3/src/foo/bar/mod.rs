fn func3() {
    println!("called foo::bar::func3()");
}

pub fn func4() {
    println!("called foo::bar::func4()");
    func3();
}
