#![allow(unused_variables, unused_mut)]
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::process;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

#[derive(Debug)]
pub struct Pair {
    key: String,
    count: u32,
}

#[derive(Debug)]
pub struct Container {
    size: usize,
    partitions: Vec<HashMap<String, Vec<Pair>>>,
}

fn exit_message(message: &str) -> ! {
    println!("[Error] : {}", message);
    process::exit(1);
}

impl Pair {
    pub fn new(key: String, count: u32) -> Self {
        Pair { key, count }
    }

    pub fn get_count(&self) -> u32 {
        self.count
    }
}

impl Container {
    pub fn new(size: usize) -> Self {
        Container {
            size,
            partitions: {
                let mut v = Vec::with_capacity(size);
                for i in 0..size {
                    v.push(HashMap::new());
                }
                v
            },
        }
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    /// returns a mutable reference to Hashmap inside the partition at index i  
    pub fn get_parition_hash_map(&self, i: usize) -> Option<&HashMap<String, Vec<Pair>>> {
        self.partitions.get(i)
    }

    /// method produces consistent hash number/index for the partition index inside the
    /// container for storing keys , value pairs
    pub fn get_partition_index(&self, key: &String) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let h = hasher.finish();
        let part = h % ((self.size) as u64);
        part as usize
    }

    ///This function is responsible for inserting (key, count) pairs in the containers for reducer
    ///threads use
    pub fn insert_pair(&mut self, pair: Pair) {
        let key: String = (pair.key).clone();
        let index = self.get_partition_index(&key);
        match self.partitions[index].get_mut(&key) {
            Some(mut vec) => {
                vec.push(pair);
            }
            // key is not present therefore create new one
            _ => {
                self.partitions[index].insert(key, vec![pair]);
            }
        };
    }

    pub fn store(&mut self, m: HashMap<String, u32>) {
        let mut tmp: Vec<Pair> = Vec::new();
        for (key, value) in m {
            tmp.push(Pair::new(key, value));
        }

        for pair in tmp.into_iter() {
            self.insert_pair(pair);
        }
    }
}
