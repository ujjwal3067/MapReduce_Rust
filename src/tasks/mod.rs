#![allow(unused_must_use)]
use crate::container;
use std::collections::HashMap;
use std::collections::VecDeque;
/// Tasks is to take filename as input and emit (key, count) to container
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub enum State {
    DONE,
    PROGRESSING,
}
pub struct MapperState {
    done: State,
}
pub struct ReducerState {
    done: State,
}

pub struct TaskQueue<T> {
    queue: VecDeque<T>,
}

// let mut MAP : MapperState = MapperState{ done : State::PROGRESSING};
// let mut REDUCE : ReducerState = ReducerState{ done : State::PROGRESSING};

impl<T> TaskQueue<T> {
    pub fn new(task_list: Vec<T>) -> Self {
        let queue: VecDeque<T> = VecDeque::from_iter(task_list);
        TaskQueue { queue }
    }

    pub fn get_task(&mut self) -> Option<T> {
        let value = self.queue.pop_front();
        match value {
            Some(_) => value,
            None => None,
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


/// takes file name as argument and calculate frequency of each word in the file
/// before emitting it to container for storage
pub fn map(filename: String, container : &mut container::Container) {
    let mut m: HashMap<String, u32> = HashMap::new(); // keeps count of each key read in this file
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            // split strings into tokens
            if let Ok(l) = line {
                let v: Vec<&str> = l
                    .split(|c: char| !c.is_alphabetic())
                    .filter(|s| s.len() >= 2)
                    .collect();
                if v.len() > 0 {
                    for s in v {
                        if !m.contains_key(s) {
                            m.insert(s.to_string(), 1);
                        } else {
                            *m.get_mut(s).unwrap() += 1;
                        }
                    }
                }
            }
        }
    }
    // store collector pairs inside the container
    container.store(m); 
}

/// takes the key value pairs inside container and start counting the frequency of each word in
/// the partition  
pub fn reducer(partition: usize, container: &mut container::Container) {
    let mut collector: HashMap<String, u32> = HashMap::new();
    let mapper_collection: &mut HashMap<String, Vec<container::Pair>> =
        container.get_parition_hash_map(partition).unwrap();
    // NOTE : key : String and  vector : &mut Vec<Pair>  
    for ( key , vector ) in mapper_collection { 
        // key not present yet 
        if !collector.contains_key(key){ 
            collector.insert(key.to_string(), 0);
        }
        for v in vector.into_iter(){ 
            *collector.get_mut(key).unwrap() += v.get_count(); 
        }
    }
    println!("after reducer : {:?}", collector); 
}
