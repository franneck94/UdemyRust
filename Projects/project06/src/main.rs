use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let status_cloned = Arc::clone(&status);

    thread::spawn(move || {
        let status = Arc::clone(&status);
        let mut status_num = status.lock().unwrap();

        for _ in 0..10 {
            thread::sleep(Duration::from_millis(250));
            status_num.jobs_completed += 1;
        }
    });

    while status_cloned.lock().unwrap().jobs_completed < 10 {
        println!("waiting... ");
        thread::sleep(Duration::from_millis(500));
    }
}
