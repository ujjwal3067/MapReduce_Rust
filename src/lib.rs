#![allow(unused_imports, unused_mut, dead_code, unused_variables)]
mod container;
mod tasks;
mod threadpool;
mod user_input;
use std::fs;
use std::sync::Mutex;
use std::sync::Arc;
use std::thread;
use std::time;
use threadpool::pool;
use std::fs::File;
use std::collections::HashMap; 

// external dependencies
use once_cell::sync::Lazy;

//global singleton container
static CONTAINER: Lazy<Mutex<container::Container>> = Lazy::new(|| {
    let mut container = container::Container::new(10);
    Mutex::new(container)
});

// global tasks queue for mapper threads
// HACK : currently not being used
static MAP_TASK_QUEUE: Lazy<tasks::TaskQueue<String>> = Lazy::new(|| {
    let mut queue = tasks::TaskQueue::new();
    queue
});

static OUTPUT_MAP : Lazy<Arc<Mutex<HashMap<String, u32>>>>  = Lazy::new(|| { 
    let mut hashmap = Arc::new(Mutex::new(HashMap::new())); 
    hashmap
});

static OUTPUT_FILE : Lazy<String> = Lazy::new(|| { 
    "./output/final_result.txt".to_string()
});


fn extract_files_names(dir: String) -> Vec<String> {
    let paths = fs::read_dir(dir).unwrap();
    let mut names: Vec<String> = vec![];
    for path in paths {
        let filepath = path.unwrap().path().to_string_lossy().to_string();
        names.push(filepath);
    }
    names
}

fn run() {
    let args = user_input::user_arguments::user_args();
    let mapper_pool = pool::MapperPool::new(args.mapper_threads as usize);
    let reducer_pool = pool::ReducerPool::new(args.mapper_threads as usize);
    let dir = args.dir;
    let filenames = extract_files_names(dir);
    println!("all files are : {:#?}", filenames);
    mapper_pool.start_executing_jobs(filenames);
    thread::sleep(time::Duration::from_secs(4));
    reducer_pool.start_executing_jobs();
    thread::sleep(time::Duration::from_secs(6));
}
