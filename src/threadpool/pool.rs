// Each mapper thread picks up a tasks and executes it and after it's done executing it goes back
// to idle state

use crate::container;
use crate::tasks;
use crate::threadpool;
use std::sync::mpsc;
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
pub struct Threadpool {
    workers: Vec<Worker>,
}

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
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id));
        }
        Threadpool { workers }
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize) -> Worker {
        let thread = thread::spawn(|| {});
        Worker { id, thread }
    }
}
