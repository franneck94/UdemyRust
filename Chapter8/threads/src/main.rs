use std::{thread, time::Duration};

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..20 {
            println!("{}", i);
            thread::sleep(Duration::from_millis(100));
        }
    });

    handle.join().unwrap();

    for i in 1..20 {
        println!("{}", i);
        thread::sleep(Duration::from_millis(100));
    }
}
