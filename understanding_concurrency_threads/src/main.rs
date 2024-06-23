use std::{thread, time::Duration};

fn main(){
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // the main thread will wait until the spwan thread is done
    handle.join().unwrap();

    for i in 1..5{
        println!("hi number {} from thje main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // the spawn thread is ended when the main thread is ended no matter what


    // the spawned thread is allow to finish
    // handle.join().unwrap();

    // example 2
    let v = vec![1,2,3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);    
    });

    handle.join().unwrap();
}