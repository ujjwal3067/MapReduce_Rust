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
#[derive(Debug)]
pub struct MapperPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

#[derive(Debug)]
pub struct ReducerPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

enum Message {
    NewJob(JobsType),
    Terminate,
}

enum JobsType { 
    Mapper(MapJob), 
    Reducer(ReducerJob), 
}

pub struct MapJob {
    job_executor: Box<dyn FnOnce(String) + Send + 'static>,
    argument: String,
}

pub struct ReducerJob { 
    job_executor : Box<dyn FnOnce(usize) + Send + 'static>, 
    argument : usize,  
}

impl MapJob {
    fn new(job: Box<dyn FnOnce(String) + Send + 'static>, args: String) -> Self {
        MapJob {
            job_executor: job,
            argument: args,
        }
    }
}

impl ReducerJob { 
    fn new (job : Box<dyn FnOnce(usize) + Send + 'static>, args : usize) -> Self { 
        ReducerJob { 
            job_executor : job, 
            argument : args, 
        }
    }
}

//type Job = (Box<dyn FnOnce() + Send + 'static>, String) ;

impl MapperPool {
    /// Creates a new threadpool
    ///
    /// The size is the number of threads in the pool
    ///
    /// # panics
    ///
    /// The `new` function will panic is the size <= 0  
    pub fn new(size: usize) -> MapperPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        MapperPool { workers, sender }
    }

    fn execute<F>(&self, f: F, filename: String)
    where
        F: FnOnce(String) + Send + 'static,
    {
        let job = MapJob::new(Box::new(f), filename);
        //self.sender.send(Message::NewJob(job)).unwrap();
        self.sender.send(Message::NewJob(JobsType::Mapper(job))); 
    }

    pub fn start_executing_jobs(&self, filenames: Vec<String>) {
        for file in filenames.into_iter() {
            self.execute(tasks::map, file);
        }
    }
}

impl ReducerPool {
    pub fn new(size: usize) -> ReducerPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ReducerPool { workers, sender }
    }

    fn execute<F>(&self, f : F , partition_num : usize) 
    where 
        F : FnOnce(usize) + Send + 'static, 
    { 
        let job =  ReducerJob::new(Box::new(f), partition_num);
        //self.sender.send(Message::NewJob)
        self.sender.send(Message::NewJob(JobsType::Reducer(job))); 
    }

    pub fn start_executing_jobs(&self) { 
        let size = crate::CONTAINER.lock().unwrap().get_size() ; 
        for i in 0..size { 
            println!("executing reducer for partition : {:?}", i); 
            self.execute(tasks::reducer, i); 
        }
    }
}

impl Drop for MapperPool {
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

#[derive(Debug)]
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
                    Message::NewJob(JobsType::Mapper(job)) => {
                        let exe = job.job_executor;
                        let arg = job.argument;
                        exe(arg);
                    },
                    Message::NewJob(JobsType::Reducer(job)) => { 
                        let exe = job.job_executor; 
                        let arg = job.argument ; 
                        exe(arg); 
                    },
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
