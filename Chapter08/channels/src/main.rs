use std::sync::mpsc;
use std::thread;

fn main() {
    let (transmitter, reciever) = mpsc::channel();
    let transmitter2 = transmitter.clone();

    thread::spawn(move || {
        let msg = String::from("My name is ");
        transmitter.send(msg).unwrap(); // moved value
    });

    thread::spawn(move || {
        let msg = String::from("Jan");
        transmitter2.send(msg).unwrap(); // moved value
    });

    let recieved_msg1 = reciever.recv().unwrap(); // waits here
    let recieved_msg2 = reciever.recv().unwrap(); // waits here

    // let recieved_msg = reciever.try_recv().unwrap(); // does not wait here

    println!("Got: {}", recieved_msg1);
    println!("Got: {}", recieved_msg2);
}
