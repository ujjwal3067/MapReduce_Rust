#![allow(unused_variables, unused_mut)]
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::process;

#[derive(Debug)]
pub struct Pair {
    key: String,
    count: u64,
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
    pub fn new(key: String, count: u64) -> Self {
        Pair { key, count }
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

    /// C Code
    ///```C
    /// unsigned long MR_DefaultHashPartition(char *key, int num_partitions) {
    ///    unsigned long hash = 5381 ;
    ///     int c;
    ///     while((c  = *key++) != '\0')
    ///         hash = hash * 33 + c;
    ///     return hash % num_partitions;
    /// }
    ///```

    pub fn get_partition(&self, key: &String) -> usize {
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
        let index = self.get_partition(&key);
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
}
