/// FnOnce: Moves, takes ownership
/// FnMut: Can change variables, mutably borrowing
/// Fn: Borrows values immutably

fn take_closure<F: Fn(i32)>(d: Vec<i32>, f: F) {
    for val in d {
        f(val)
    }
}

fn take_closure2<F: FnMut(&i32)>(d: Vec<i32>, f: F) {
    d.iter().for_each(f)
}

fn main() {
    let x = vec![1, 2, 3];
    let f = |v: i32| println!("{}", v);
    take_closure(x, f);

    let x2 = vec![1, 2, 3];
    let f2 = |v: &i32| println!("{}", v);
    take_closure2(x2, f2);
}
