/// Fn: Closures that can only borrow the
/// captured variables immutably. It means that the closure can't
/// modify the captured variables.
///
/// FnMut: Closures that can mutate the captured
/// variables. These closures have the ability to change the state of
/// variables they've captured.
///
/// FnOnce: Closure that consume the variables, they move the captured variable.
///
/// Function pointers implement all three of the closure
/// traits (Fn, FnMut, and FnOnce), meaning you can always
/// pass a function pointer as an argument for a function
/// that expects a closure.

fn main() {
    let mut count = 0;

    let print = || println!("{count} This is a Fn.");
    print();

    let mut increment = || {
        count += 1;
        println!("count: {}", count);
        println!("This is a FnMut.")
    };

    increment();
}
