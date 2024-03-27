// Do not communicate by sharing memory; instead, share memory by communicating
// A channel is a general programming concept by which data is sent from one thread to another.
use std::sync::mpsc;
use std::thread;

fn main() {
    let (transmitter1, reciever1) = mpsc::channel();
    let transmitter2 = transmitter1.clone();

    let t1_handle = thread::spawn(move || {
        let msg = String::from("Hello From Thread T1");
        transmitter1.send(msg).unwrap();

        // DO SOME STUFF

        let msg = String::from("Goodbye From Thread T1");
        transmitter1.send(msg).unwrap();
    });

    let t2_handle = thread::spawn(move || {
        let msg = String::from("Hello From Thread T2");
        transmitter2.send(msg).unwrap();

        // DO SOME STUFF

        let msg = String::from("Goodbye From Thread T2");
        transmitter2.send(msg).unwrap();
    });

    // let recieved_msg1 = reciever1.recv().unwrap();
    // println!("Recieved1: {recieved_msg1}");
    // let recieved_msg2 = reciever1.recv().unwrap();
    // println!("Recieved2: {recieved_msg2}");

    for recieved_msg in reciever1.iter() {
        println!("Recieved: {recieved_msg}");
    }

    t1_handle.join().unwrap();
}
