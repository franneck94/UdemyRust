use std::sync::mpsc;
use std::thread;

fn main() {
    let (transmitter, reciever) = mpsc::channel();
    let transmitter2 = transmitter.clone();

    thread::spawn(move || {
        let msg = String::from("My name is ");
        transmitter.send(msg).unwrap();
    });

    thread::spawn(move || {
        let msg = String::from("Jan ");
        transmitter2.send(msg).unwrap();
    });

    let recieved_msg1 = reciever.recv().unwrap();
    let recieved_msg2 = reciever.recv().unwrap();

    // let recieved_msg3 = reciever.try_recv().unwrap();

    println!("Recieved: {}{}", recieved_msg1, recieved_msg2);
}
