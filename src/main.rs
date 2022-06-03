#![allow(unused_imports, unused_mut, dead_code, unused_variables)]
mod container;
mod tasks;
mod threadpool;
mod user_input;
use std::sync::Mutex;
use std::thread;
use std::time;
use threadpool::pool;

// external dependencies
use once_cell::sync::Lazy;

//global singleton container
static CONTAINER: Lazy<Mutex<container::Container>> = Lazy::new(|| {
    let mut container = container::Container::new(10);
    Mutex::new(container)
});

// global tasks queue for mapper threads
static MAP_TASK_QUEUE: Lazy<tasks::TaskQueue<String>> = Lazy::new(|| {
    let mut queue = tasks::TaskQueue::new();
    queue
});

fn main() {
    let args = user_input::user_arguments::user_args();
    let pool = pool::Threadpool::new(args.mapper_threads as usize);
    let filenames = args.filesnames; 
    pool.start_executing_jobs(filenames);
    // sleep main thread to see the result of container after tasks are executed
    thread::sleep(time::Duration::from_secs(3)); 
    println!("{:#?}", CONTAINER);
}
