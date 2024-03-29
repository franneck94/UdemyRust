// Do not communicate by sharing memory; instead, share memory by communicating
// A channel is a general programming concept by which data is sent from one thread to another.
use std::sync::mpsc; // multiple producer, single consumer
use std::thread;
use std::time::Duration;

// Main Thread is T0
fn main() {
    let (transmitter, reciever) = mpsc::channel();

    thread::spawn(move || {
        let msg = String::from("Hello from Thread T1");
        transmitter.send(msg).unwrap();

        // DO SOME STUFF
        thread::sleep(Duration::from_millis(3_000));
        let msg = String::from("... still computing ...");
        transmitter.send(msg).unwrap();
        thread::sleep(Duration::from_millis(3_000));

        let msg = String::from("Goodbye from Thread T1");
        transmitter.send(msg).unwrap();
    });

    // let recieved_message1 = reciever.recv().unwrap();
    // println!("Recieved1: {recieved_message1}");
    // let recieved_message2 = reciever.recv().unwrap();
    // println!("Recieved2: {recieved_message2}");

    for recieved_message in reciever.iter() {
        println!("Recieved: {recieved_message}");
    }
}
