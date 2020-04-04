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
                    lock.push_str("Patatas");
                    println!("{}", &lock);
                })
            },
            {
                let data = data.clone();
                thread::spawn(move || {
                    sleep(Duration::from_secs(2));
                    let lock = data.lock().expect("Potato");
                    println!("{}", &lock);
                })
            },
        ]
    };

    for thread in threads {
        thread.join();
    }
}
