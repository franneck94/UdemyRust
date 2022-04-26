// every reference has a life time, to prevent dangling references
// life time annotations do not modify the life time, just describing the relationships
// 'a: lifetime (spoken: ticked a)
// each parameter can have its own ticked lifetime

fn main() {
    // let r: &str;
    // {
    //     let string1 = String::from("test");
    //     r = &string1;
    // }
    // println!("{}", r);

    let string1 = String::from("test");
    let string2 = String::from("testtt");

    let longest_string = longest(&string1, &string2);
    println!("{}", longest_string);
}

// return value lifetime must math the input parameter lifetime
// Reason: only can return reference if its still valid after function ends
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
