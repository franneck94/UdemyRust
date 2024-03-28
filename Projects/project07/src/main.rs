mod lib;

use std::sync::mpsc;
use std::thread;

struct WorkerResult {
    thread_id: u64,
    number: u64,
    is_prime: bool,
}

fn worker(
    transmitter: mpsc::Sender<WorkerResult>,
    thread_id: u64,
    start_offset: u64,
    end_offset: u64,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        for n in start_offset..end_offset {
            let result = lib::is_prime(n);

            transmitter
                .send(WorkerResult {
                    thread_id,
                    number: n,
                    is_prime: result,
                })
                .unwrap();
        }
    })
}

fn number_split(thread_id: u64) -> (u64, u64) {
    let start_value = 500_000_000;
    let range_size = 10_000;
    let offset_start = start_value + thread_id * range_size;
    let offset_end = offset_start + range_size;

    (offset_start, offset_end)
}

fn main() {
    let mut handles = vec![];

    let reciver = {
        let (transmitter, reciver) = mpsc::channel();

        for thread_id in 0..10 {
            let (offset_start, offset_end) = number_split(thread_id);
            let transmitter = transmitter.clone();
            let handle = worker(transmitter, thread_id, offset_start, offset_end);

            handles.push(handle);
        }

        reciver
    };

    for (message_idx, recieved_message) in reciver.iter().enumerate() {
        if recieved_message.is_prime {
            println!(
                "{message_idx} - Thread: {}, {} is prime {}",
                recieved_message.thread_id, recieved_message.number, recieved_message.is_prime
            );
        }
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
