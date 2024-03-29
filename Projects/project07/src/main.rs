mod lib;

use std::sync::mpsc;
use std::thread;
use std::time::Instant;

struct WorkerResult {
    thread_id: u64,
    results: Vec<(u64, bool)>,
}

fn worker(
    transmitter: mpsc::Sender<WorkerResult>,
    thread_id: u64,
    start_offset: u64,
    end_offset: u64,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let mut results: Vec<(u64, bool)> = vec![];

        for n in start_offset..end_offset {
            let result = lib::is_prime(n);

            results.push((n, result));
        }

        transmitter
            .send(WorkerResult { thread_id, results })
            .unwrap();
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

    let start_time = Instant::now();

    let reciever = {
        let (transmitter, reciever) = mpsc::channel();

        for thrad_id in 0..10 {
            let (offset_start, offset_end) = number_split(thrad_id);
            let transmitter = transmitter.clone();
            let handle = worker(transmitter, thrad_id, offset_start, offset_end);

            handles.push(handle);
        }

        reciever
    };

    for (message_idx, recieved_message) in reciever.iter().enumerate() {
        for (number, is_prime) in recieved_message.results {
            if is_prime {
                println!(
                    "{message_idx} - Thread: {}, {} is prime {}",
                    recieved_message.thread_id, number, is_prime
                );
            }
        }
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let end_time = Instant::now();
    let elapsed_time = end_time - start_time;
    println!("Elapsed Time: {elapsed_time:?}");
}
