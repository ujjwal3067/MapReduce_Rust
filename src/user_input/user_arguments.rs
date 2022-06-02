#![allow(dead_code)]
use std::env ; 
use std::process; 

/// This private struct  is responsible for collecting arguments for the program  
/// arg1 : number of mapper threads 
/// arg2 : number of reducer threads
/// arg > 2 : filesnames for map function  

struct Args { 
    mapper_threads : u32, 
    reducer_threads : u32, 
    filesnames  : Vec<String>, 
}

impl Args { 
    fn new(mapper_threads : u32 , reducer_threads : u32) -> Self { 
        Args { 
            mapper_threads, 
            reducer_threads, 
            filesnames  : Vec::new(), 
        }
    }

    fn add_filename(&mut self, filename : String) { 
        self.filesnames.push(filename); 

    }
}

fn exit_message(message : &str) -> ! { 
    println!("[Error] : {}", message);
    help(); 
    process::exit(1); 
}

fn help(){ 
    println!("usage : 
    $./process  mapper_threads reducer_threads [files...] \n
    NOTE : mapper threads and reducer threads  are bounded by 10 threads each 
     "); 

}
pub fn user_args() { 
    let args : Vec<String> = env::args().collect(); 
    match args.len( ){ 
        1..=2 => { 
            exit_message("not enough arguments");
        }, 
        i if i >=3  => { 
            let mapper_thread = match args[1].parse() { 
                Ok(x) =>x , 
                _ => exit_message("thread count is not an integer"), 
            };
        }
        _ => { 
            println!("error : Wrong arguments");
            help(); 
            process::exit(1); 
        }
    }

}
