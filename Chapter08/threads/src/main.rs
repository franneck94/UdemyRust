use std::{thread, time::Duration};

// t0
fn main() {
    // t1
    let handle_t1 = thread::spawn(|| {
        for i in 1..=10 {
            println!("T1: {i}");
            thread::sleep(Duration::from_millis(100))
        }
    });

    // DO SOME STUFF
    println!("SOME STUFF!");
    for i in 1..=10 {
        println!("T0: {i}");
        thread::sleep(Duration::from_millis(100))
    }

    handle_t1.join().unwrap();

    println!("AFTER!");

    let v = vec![1, 2, 3];
    // use the move keyword with closures passed to thread::spawn because the closure will
    // then take ownership of the values it uses from the environment
    let handle_t2 = thread::spawn(move || {
        for val in v {
            println!("T2: {val}");
            thread::sleep(Duration::from_millis(100))
        }
    });

    handle_t2.join().unwrap();
}
