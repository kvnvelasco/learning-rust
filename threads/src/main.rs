use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let data = Arc::new(Mutex::new("Some Data".to_owned()));

    let threads = {
        vec![
            {
                let data = data.clone();
                thread::spawn(move || {
                    let mut lock = data.lock().expect("Potato");
                    // prevent the next thread from acquiring the lock synthetically;
                    sleep(Duration::from_secs(2));
                    lock.push_str("Patatas");
                    println!("thread 1: {}", &lock);
                })
            },
            {
                let data = data.clone();
                thread::spawn(move || {
                    // this lock will wait for the thread above to drop it's lock (go out of scope)
                    // before it's granted. This can cause deadlocks if not implemented correctly.
                    let lock = data.lock().expect("Potato");
                    println!("Thread 2: {}", &lock);
                })
            },
        ]
    };

    for thread in threads {
        thread.join();
    }
}
