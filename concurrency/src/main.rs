use std::{thread, time::Duration, };
use std::sync::mpsc; 

fn main() {

    let handle= thread::spawn(|| {

        for i in 1..10 {
            println!("number spawned thread: {}", i);
            thread::sleep(Duration::from_millis(1));
        }

    }); 

    handle.join(); 
    

    let (tx , rx ) = mpsc::channel(); 

    thread::spawn(move || {
        let msg = String::from("hi"); 
        tx.send(msg).unwrap();
    });

    let received = rx.recv().unwrap(); 




}
