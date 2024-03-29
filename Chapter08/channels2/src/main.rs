use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (transmitter1, reciever) = mpsc::channel();
    let transmitter2 = transmitter1.clone();

    thread::spawn(move || {
        let msg = String::from("Hello from Thread T1");
        transmitter1.send(msg).unwrap();

        // DO SOME STUFF
        thread::sleep(Duration::from_millis(2_000));
        let msg = String::from("... still computing ...");
        transmitter1.send(msg).unwrap();
        thread::sleep(Duration::from_millis(1_000));

        let msg = String::from("Goodbye from Thread T1");
        transmitter1.send(msg).unwrap();
    });

    thread::spawn(move || {
        let msg = String::from("Hello from Thread T2");
        transmitter2.send(msg).unwrap();

        // DO SOME STUFF
        thread::sleep(Duration::from_millis(1_000));
        let msg = String::from("... still computing ...");
        transmitter2.send(msg).unwrap();
        thread::sleep(Duration::from_millis(1_000));

        let msg = String::from("Goodbye from Thread T2");
        transmitter2.send(msg).unwrap();
    });

    for recieved_message in reciever.iter() {
        println!("Recieved: {recieved_message}");
    }
}
