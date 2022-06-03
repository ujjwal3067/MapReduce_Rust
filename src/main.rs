#![allow(unused_imports, dead_code, unused_variables)]
mod container;
mod tasks;
mod threadpool;
mod user_input;
use std::thread;
use std::time;

fn main() {
    // let mut container = container::Container::new(10);
    // tasks::map("test_resource/file1.txt".to_string(), &mut container);
    // for i in 0..container.get_size() {
    //     println!("-----------------------------------------------------------------------");
    //     println!("partition : {}", i);
    //     tasks::reducer(i, &mut container);
    //     println!("-----------------------------------------------------------------------");
    // }

    // TESING threadpool

    use threadpool::pool;
    let pool = pool::Threadpool::new(10);
    for i in 0..1000 {
        // Captures the variables i by moving it into the closure
        pool.execute(move || {
            println!("running : {}", i);
        });
    }
    //BUG : not all the jobs are finished before programs terminates
    thread::sleep(time::Duration::from_secs(3));
}
