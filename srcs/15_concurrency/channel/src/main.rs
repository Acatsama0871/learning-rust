use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // tx and rx example
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let string = String::from("Hello");
        tx.send(string).unwrap();
    });
    let received = rx.recv().unwrap();
    println!("{}", received);
    
    
    // send the value one by one
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        let values = vec![
            String::from("hi"),
            String::from("how"),
            String::from("are"),
            String::from("you")
        ];
        
        for cur_val in values {
            tx.send(cur_val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    for r in rx {
        println!("Got {}", r);
    }
    
    // multiple sender one receiver
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        
        for v in vals {
            tx.send(v).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("another"),
            String::from("thread")
        ];
        
        for v in vals {
            tx1.send(v).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    
    for r in rx {
        println!("Received {}", r);
    }
}
