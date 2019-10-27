use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::Mutex;

fn main() {

    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);

    let handle = thread::spawn(move || {
       for i in 1..10 {
           tx.send(format!("hi number {} from spawned thread", i)).unwrap();
           thread::sleep(Duration::from_secs(2));
       } 
    });

    thread::spawn(move || {
       for i in 1..10 {
           tx1.send(format!("hi number {} from spawned thread tx1", i)).unwrap();
           thread::sleep(Duration::from_secs(2));
       } 
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

fn mutex_demo() {
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 4;
    }
    println!("m = {:?}", m);
}