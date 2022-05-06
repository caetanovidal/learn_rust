use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..3 {
        println!("Hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(3));
    }

    handle.join().unwrap();

    println!("=====================================================");

    let v = vec![1, 2, 3];

    let handl4 = thread::spawn(move || {
        println!("here is a vector {:?}", v);
    });

    // drop(v);

    handl4.join().unwrap();

    println!("================================================");

    
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
            
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(2));
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
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for receive in rx {
        println!("Got: {} ", receive)
    }    

}
