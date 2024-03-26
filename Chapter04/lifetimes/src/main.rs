// every reference has a lifetime
// lifetime generic annotations do not modify lifetime
// lifetimes ensure that references are valid as long as we need them to be
// generic lifetime parameters define the relationship between the references
// 'a: lifetime (spoken: tick a)

fn invalid_lifetime() {
    let r: &str;

    {
        let string1 = String::from("test");
        r = &string1;
    }

    println!("{}", r);
}

fn valid_lifetime() {
    let string1 = String::from("test1");
    let string2 = String::from("test2");

    // {
    //     let result = longest_str_invalid(&string1, &string2);
    //     println!("{}", result);
    // }

    {
        let result = longest_str_valid(&string1, &string2);
        println!("{}", result);
    }
}

fn longest_str_invalid(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest_str_valid<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // invalid_lifetime();
    valid_lifetime();
}
