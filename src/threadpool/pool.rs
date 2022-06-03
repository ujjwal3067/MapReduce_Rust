// Each mapper thread picks up a tasks and executes it and after it's done executing it goes back
// to idle state

use crate::container;
use crate::tasks;
use crate::threadpool;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
/// we want the worker structs to fetch jobs from a queue that the Threadpool  holds and run
/// and run these jobs
///
/// [basic logic]
/// Threadpool holds sending side of the channel
/// Worker thread holds receving side of the channel
///
///A new job struct will hold the closure we want to send down the channel
/// and then execute method of the threadpool will send the job it wants to execute down the
/// sending side of the channel
/// In a thread, the worker will loop over its receiving side of the channel and execute the
/// closures of any jobs it executes.
/// NOTE : receiver is shared between mutlipler worker thread via Arc<Mutex<T>> smart pointer
pub struct Threadpool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

enum Message {
    NewJob(Job),
    Terminate,
}

/// closure  + static type  + safe to move between threads
type Job = Box<dyn FnOnce() + Send + 'static>;

impl Threadpool {
    /// Creates a new threadpool
    ///
    /// The size is the number of threads in the pool
    ///
    /// # panics
    ///
    /// The `new` function will panic is the size <= 0  
    pub fn new(size: usize) -> Threadpool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        Threadpool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}
impl Drop for Threadpool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            self.sender
                .send(Message::Terminate)
                .expect("[Error] : Failed to terminate the Worker thread with ");
        }
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().expect(
                    "[Error] : Some worker thread didn't finish before program got terminated",
                );
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                // this is blocking operation
                let message = receiver
                    .lock()
                    .unwrap()
                    .recv()
                    .expect("[Error] : There was an error on the receiver side of the channel");
                match message {
                    Message::NewJob(job) => {
                        job();
                    }
                    Message::Terminate => {
                        break;
                    }
                }
            }
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}
