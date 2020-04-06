use futures::task::{Context, Poll, Waker};
use futures::Future;
use std::borrow::{Borrow, BorrowMut};
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant, SystemTime};

struct TimerState {
    completed: bool,
    waker: Option<Waker>,
}

pub struct Timer {
    state: Arc<Mutex<TimerState>>,
}

impl Timer {
    pub fn new(duration: Duration, thread_count: Option<Arc<Mutex<usize>>>) -> Self {
        let state = Arc::new(Mutex::new(TimerState {
            completed: false,
            waker: None,
        }));

        let thread_state = state.clone();
        let start = Instant::now();
        thread::spawn(move || {
            println!("Waited {}\tns for thread", start.elapsed().as_nanos());

            if let Some(thread_count) = thread_count.borrow() {
                let mut counter = thread_count.lock().expect("Unable to lock counter");
                *counter += 1;
                println!("Spawing thread -- {} active", counter)
            }
            thread::sleep(duration);
            let mut state = thread_state.lock().expect("Unable to get thread state");
            state.completed = true;

            if let Some(waker) = state.waker.take() {
                waker.wake();
            };

            if let Some(thread_count) = thread_count.borrow() {
                let mut counter = thread_count.lock().expect("Unable to lock counter");
                *counter -= 1;
                println!("Killing thread -- {} active", counter)
            }
        });

        Timer { state }
    }
}

impl Future for Timer {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut state = self
            .state
            .lock()
            .expect("Unable to get lock for timer mutex");
        if state.completed {
            Poll::Ready(())
        } else {
            state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}
