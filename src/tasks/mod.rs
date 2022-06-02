#![allow(unused_must_use)]
/// Tasks is to take filename as input and emit (key, count) to container

use std::fs::File; 
use std::io::{self, BufRead}; 
use std::path::Path;
use std::collections::VecDeque; 
use std::collections::HashMap; 
use crate::container ; 


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
    queue : VecDeque<T>,
}

// let mut MAP : MapperState = MapperState{ done : State::PROGRESSING};
// let mut REDUCE : ReducerState = ReducerState{ done : State::PROGRESSING};

impl <T>TaskQueue<T>{ 
    pub fn new(task_list : Vec<T>) -> Self {
        let queue : VecDeque<T> = VecDeque::from_iter(task_list); 
        TaskQueue{ 
            queue
        }
    }

    pub fn get_task(&mut self) -> Option<T> { 
        let value = self.queue.pop_front(); 
        match value  { 
            Some(_) => value, 
            None =>  { 
                None
            }
        }
    } 
}

fn read_lines<P>(filename : P) -> io::Result<io::Lines<io::BufReader<File>>> 
where P : AsRef<Path>, 
{
    let file = File::open(filename)?; 
    Ok(io::BufReader::new(file).lines())

}


fn emit() { 

}
pub fn map(filename: String){
    let mut m  : HashMap<String, u32> = HashMap::new();  // keeps count of each key read in this file
    if let Ok(lines)  = read_lines(filename) { 
        for line in lines { 
            // split strings into tokens
            if let Ok(l) = line { 
                let v : Vec<&str> = l.split(|c : char| !c.is_alphabetic()).filter(|s| s.len() >= 2).collect();
                if v.len() > 0 { 
                    for s in v { 
                        if !m.contains_key(s) { 
                            m.insert(s.to_string(), 1);
                        }else { 
                            *m.get_mut(s).unwrap() += 1 ; 
                        }
                    }
                }
            }
        }
    }

    println!("hashmap =  {:#?}", m); 
}

pub fn reducer() {

}
