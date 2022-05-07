/// - *FnOnce* moves the variables it captures, can’t take ownership of the same variables more than once
/// - *FnMut* can change the environment because it mutably borrows values
/// - *Fn* borrows values from the environment immutably

fn takes_closure<F: FnMut(&i32)>(d: &[i32], f: F) {
    d.iter().for_each(f);
}

fn main() {
    let x = vec![1, 2, 3];

    let f = |&v: &i32| println!("{}", v);

    takes_closure(&x, f);

    println!("{:?}", x);
}
