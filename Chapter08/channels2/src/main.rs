use std::sync::mpsc;
use std::thread;

fn main() {
    let (transmitter1, reciever1) = mpsc::channel();
    let transmitter2 = transmitter1.clone();

    thread::spawn(move || {
        let msg = String::from("Hello From Thread T1");
        transmitter1.send(msg).unwrap();

        // DO SOME STUFF

        let msg = String::from("Goodbye From Thread T1");
        transmitter1.send(msg).unwrap();
    });

    thread::spawn(move || {
        let msg = String::from("Hello From Thread T2");
        transmitter2.send(msg).unwrap();

        // DO SOME STUFF

        let msg = String::from("Goodbye From Thread T2");
        transmitter2.send(msg).unwrap();
    });

    for recieved_msg in reciever1.iter() {
        println!("Recieved: {recieved_msg}");
    }
}
