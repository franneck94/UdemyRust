mod my;

fn function() {
    println!("called `function()`");
}

fn main() {
    function();

    my::function();

    my::indirect_access();

    my::nested::indirect_access();
}
