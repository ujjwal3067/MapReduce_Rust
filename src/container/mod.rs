#![allow(unused_variables, unused_mut)]
use std::collections::HashMap; 
use std::collections::hash_map::DefaultHasher; 
use std::hash::{Hash, Hasher}; 

pub struct Pair { 
    key : String, 
    count : u32, 
}

pub struct Container{ 
    size : u32, 
    partitions : Vec<HashMap<String, Vec<Pair>>>, 
} 

impl Container { 

    pub fn new(size : u32) -> Self { 
        Container { 
            size , 
            partitions : Vec::new(), 
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


    pub fn get_partition(&self, key : String) -> usize { 
        let mut hasher = DefaultHasher::new() ;
        key.hash(&mut hasher); 
        let h  = hasher.finish(); 
        let part   =  h % ((self.size) as u64 ) ; 
        part as usize
    }
}


