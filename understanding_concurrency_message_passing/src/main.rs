use std::sync::mpsc; // multi producer single consumer
use std::{thread, time::Duration};

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();

    // thread::spawn(move || {
    //     let msg = String::from("hi");
    //     tx.send(msg).unwrap(); // can't modify after send thru channel
    // });

    // let received = rx.recv().unwrap();
    // println!("Got: {}", received);

    // example 2
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
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
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
