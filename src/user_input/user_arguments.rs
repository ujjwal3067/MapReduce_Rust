#![allow(dead_code, unused_variables)]
use std::env;
use std::fmt;
use std::process;

/// This private struct  is responsible for collecting arguments for the program  
/// arg1 : number of mapper threads
/// arg2 : number of reducer threads
/// arg > 2 : filesnames for map function  

#[derive(Debug)]
pub struct Args {
    pub mapper_threads: u32,
    pub reducer_threads: u32,
    pub filesnames: Vec<String>,
}

/// Note used at the moment
impl fmt::Display for Args {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "mapper threads count : {}\n reducer thread count {}\n files name : {:?}",
            self.mapper_threads, self.reducer_threads, self.filesnames
        )
    }
}

impl Args {
    fn new(mapper_threads: u32, reducer_threads: u32, files: &[String]) -> Self {
        Args {
            mapper_threads,
            reducer_threads,
            filesnames: files.to_vec(),
        }
    }
}

fn exit_message(message: &str) -> ! {
    println!("[Error] : {}", message);
    help();
    process::exit(1);
}

fn help() {
    println!(
        "usage : 
    $./process  mapper_threads reducer_threads [files...] \n
    [NOTE]  mapper threads and reducer threads  are bounded by 10 threads each 
     "
    );
}

pub fn user_args() -> Args {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1..=2 => {
            exit_message("not enough arguments");
        }
        i if i >= 3 => {
            let arg1 = &args[1];
            let arg2 = &args[2];
            let mapper_threads = match arg1.parse::<i32>() {
                Ok(x) => x,
                _ => exit_message("(while parsing i32) thread count is not an integer"),
            };
            let reducer_threads = match arg2.parse::<i32>() {
                Ok(x) => x,
                _ => exit_message("(while parsing i32) thread count is not an integer"),
            };
            let files = &args[3..];
            Args::new(mapper_threads as u32, reducer_threads as u32, files)
        }
        _ => {
            println!("error : Wrong arguments");
            help();
            process::exit(1);
        }
    }
}
