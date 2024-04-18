// use statements
use std::sync::mpsc;
use std::thread;
use std::time::Duration;


fn main() {
    // create transmitter and receiver elements
    let (tx, rx) = mpsc::channel();

    // clone transmitter to simulate multiple senders
    let tx1 = tx.clone();

    // create new thread
    thread::spawn(move || {

        // create vector of strings to pass from transmitter to receiver
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        // move messages to receiver using send method
        for val in vals{

            // move message to receiver
            tx1.send(val).unwrap();

            // let thread sleep for a second
            thread::sleep(Duration::from_secs(1))
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1))
        }
    });

    // get string from receiver
    //let received = rx.recv().unwrap();  // this was used for a single message
    for received in rx {
        println!("Got: {}", received);
    }
}
