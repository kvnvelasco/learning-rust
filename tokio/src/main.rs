use tokio::prelude::*;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut runtime: tokio::runtime::Runtime = tokio::runtime::Builder::new()
        .enable_all()
        .basic_scheduler()
        .threaded_scheduler()
        .on_thread_start(|| {
            println!("Thread starting")
        })
        .on_thread_stop(|| {
            println!("thread stopping");
        })
        .build()
        .expect("unable to build");

    let handle1 = runtime.spawn(async {
        sleep(Duration::from_secs(10));
        println!("Thread 1")
    });
    let handle2 = runtime.spawn(async { println!("Thread 2") });

    runtime.block_on(futures::future::join(handle1, handle2));
}
