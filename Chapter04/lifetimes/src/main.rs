// every reference has a life time
// life time annotations do not modify life time
// 'a: lifetime (spoken: tick a)

fn main() {
    // let r: &str;

    // {
    //     let string1 = String::from("test");
    //     r = &string1;
    // }

    // println!("{}", r);

    let string1 = String::from("test1");
    let string2 = String::from("test2");

    let result = longest_str(&string1, &string2);
    println!("{}", result);
}

fn longest_str<'a>(x: &'a String, y: &'a String) -> &'a String {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
