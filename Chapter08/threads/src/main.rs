use std::{thread, time::Duration};

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("{}", i);

            thread::sleep(Duration::from_millis(100));
        }

        thread::sleep(Duration::from_millis(2000));
    });

    handle.join().unwrap();

    for i in 1..10 {
        println!("{}", i);

        thread::sleep(Duration::from_millis(100));
    }

    let vec = vec![1, 2, 3];

    let handle2 = thread::spawn(move || {
        for val in vec {
            println!("{}", val);

            thread::sleep(Duration::from_millis(100));
        }
    });

    handle2.join().unwrap();

    // println!("{:?}", vec);
}
