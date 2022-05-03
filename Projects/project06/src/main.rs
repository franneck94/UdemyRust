use std::sync::mpsc;
use std::thread;

fn is_prime(n: u64) -> bool {
    if n == 0 || n == 1 {
        return false;
    }

    if (n % 2 == 0) || (n % 3 == 0) || (n % 5 == 0) || (n % 7 == 0) || (n % 11 == 0) {
        return true;
    }

    let upper = (n as f64).sqrt() as u64;

    (13..=upper).step_by(2).all(|i| n % i != 0)
}

fn worker(
    transmitter: mpsc::Sender<(u64, u64)>,
    id: u64,
    start: u64,
    end: u64,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        for n in start..end {
            let result = is_prime(n);

            if result {
                transmitter.send((id, n)).unwrap();
            }
        }
    })
}

fn main() {
    let (transmitter, reciever) = mpsc::channel();

    let mut handles = vec![];

    for id in 0..10 {
        let transmitter = transmitter.clone();

        let handle = worker(transmitter, id, 1_000_000_000, 1_000_001_000);

        handles.push(handle);
    }

    for recieved_msg in reciever {
        println!("Thread: {}, {} is prime", recieved_msg.0, recieved_msg.1);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
