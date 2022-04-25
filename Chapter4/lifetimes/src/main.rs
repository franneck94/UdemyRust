// every reference has a life time, to prevent dangling references
// life time annotations do not modify the life time, just describing the relationships
// 'a: lifetime (spoken: ticked a)
// each parameter can have its own ticked lifetime

fn main() {
    // let r;

    // {
    //     let x = 5;
    //     r = &x;
    // }

    // println!("{}", r);

    // &i32
    // &'a i32
    // &*a mut i32

    let _s = example("hi");
}

// return value lifetime must math the input parameter lifetime
// Reason: only can return reference if its still valid after function ends
fn example<'a>(x: &'a str) -> &'a str {
    x
}
