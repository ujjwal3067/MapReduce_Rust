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
    sender: mpsc::Sender<Job>,
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
        // so that receiver can be shared between multiple worker threads for picking up jobs for execution 
        let receiver = Arc::new(Mutex::new(receiver));  
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        Threadpool { workers, sender }
    }


    pub fn execute<F>(&self, f : F) 
        where 
            F : FnOnce() + Send + 'static
        { 
            let job = Box::new(f); 
            self.sender.send(job).unwrap();
        }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // we are sending recevier to worker thread ends for executing jobs
        let thread = thread::spawn(move|| {
            loop { 
                // this is blocking operation
                let job = receiver.lock().unwrap().recv().expect("[Error] : There was an error on the receiver side of the channel"); 
                println!("Worker {} got a job : executing.", id); 
                job(); // execute the closure
            }
        });
        Worker { 
            id,
            thread, 
        }
    }
}
