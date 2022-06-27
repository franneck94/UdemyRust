use std::sync::mpsc;
use std::thread;

fn is_prime(n: u64) -> bool {
    if n == 0 || n == 1 {
        return false;
    }

    if (n == 2) || (n == 3) || (n == 5) || (n == 7) || (n == 11) {
        return true;
    }

    if (n % 2 == 0) || (n % 3 == 0) || (n % 5 == 0) || (n % 7 == 0) || (n % 11 == 0) {
        return false;
    }

    let upper = (n as f64).sqrt() as u64;

    (13..=upper).step_by(2).all(|i| n % i != 0)
}

fn worker(
    transmitter: mpsc::Sender<(u64, u64, bool)>,
    id: u64,
    start: u64,
    end: u64,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        for n in start..end {
            let result = is_prime(n);

            transmitter.send((id, n, result)).unwrap();
        }
    })
}

fn number_split(id: u64) -> (u64, u64) {
    let offset_start = if id > 1 {
        1_000_000_000 + (id - 1) * 3
    } else {
        1_000_000_000
    };
    let offset_end = 1_000_000_000 + id * 3;

    (offset_start, offset_end)
}

fn main() {
    let mut handles = vec![];

    let recv = {
        let (transmitter, recv) = mpsc::channel();

        for id in 0..10 {
            let transmitter = transmitter.clone();

            let (offset_start, offset_end) = number_split(id);
            let handle = worker(transmitter, id, offset_start, offset_end);

            handles.push(handle);
        }

        recv
    };

    for (idx, recv_msg) in recv.iter().enumerate() {
        println!(
            "{} - Thread: {}, {} is prime {}",
            idx, recv_msg.0, recv_msg.1, recv_msg.2
        );
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
