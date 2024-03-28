use std::sync::{Arc, Mutex};
use std::thread;

const NUM_THREADS: i32 = 10;

fn main() {
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..NUM_THREADS {
        let cloned_counter = counter.clone();

        let handle = thread::spawn(move || {
            let mut locked_counter = cloned_counter.lock().unwrap();

            *locked_counter += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Counter: {}", counter.lock().unwrap());
}
