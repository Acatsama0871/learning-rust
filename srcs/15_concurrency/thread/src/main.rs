use std::thread;
use std::time::Duration;


fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hello from number {} in another thread", i);
            thread::sleep(Duration::from_millis(2));
        }
    });
    handle.join().unwrap();
    
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(2));
    }
    
    // pass parameters to thread
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here is the vector {:?}", v);
    });
    
    handle.join().unwrap();
}
