use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::time::SystemTime;

fn main() {
    let (mut executor, mut spawner) = executor::new_executor();

    let thread_count = Arc::from(Mutex::new(0));
    let main = thread::spawn(move || {
        executor.run();
    });

    for i in 0..=100_000 {
        let thread_count = thread_count.clone();
        spawner.spawn(async move {
            let time = SystemTime::now();
            timer::Timer::new(Duration::from_millis(100), None).await;
            // println!("Completed in: {}ms", time.elapsed().unwrap().as_millis());
            ()
        });
    }

    main.join();
}

pub mod executor {
    use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
    use std::sync::{Arc, Mutex, MutexGuard};

    use futures::future::BoxFuture;
    use futures::task::{waker_ref, ArcWake, Context, Poll};
    use futures::{Future, FutureExt};

    pub struct Executor {
        queue: Receiver<Arc<Task>>,
    }

    impl Executor {
        pub fn run(&mut self) {
            while let task = self.queue.recv().expect("Unable to get task") {
                let mut future_slot: MutexGuard<Option<BoxFuture<()>>> = task
                    .current_future
                    .lock()
                    .expect("Unable to get future from task");
                if let Some(mut fut) = future_slot.take() {
                    let waker = waker_ref(&task);
                    let mut cx = Context::from_waker(&*waker);

                    if fut.as_mut().poll(&mut cx).is_pending() {
                        *future_slot = Some(fut);
                    }
                }
            }
        }
    }

    // adds tasks to the executor somehow with a handle
    pub struct Spawner {
        spawn_handle: SyncSender<Arc<Task>>,
    }

    impl Spawner {
        pub fn spawn(&mut self, future: impl Future<Output = ()> + 'static + Send) {
            let boxed_future = future.boxed();
            let task = Arc::new(Task {
                current_future: Mutex::new(Some(boxed_future)),
                requeue_handle: self.spawn_handle.clone(),
            });

            self.spawn_handle.send(task).expect("Unable to send task");
        }
    }

    pub struct Task {
        current_future: Mutex<Option<BoxFuture<'static, ()>>>,
        requeue_handle: SyncSender<Arc<Task>>,
    }

    impl ArcWake for Task {
        fn wake_by_ref(arc_self: &Arc<Self>) {
            arc_self.requeue_handle.send(arc_self.clone());
        }
    }

    pub fn new_executor() -> (Executor, Spawner) {
        let (spawn_handle, queue) = sync_channel(10_000);
        (Executor { queue }, Spawner { spawn_handle })
    }
}

mod timer;
