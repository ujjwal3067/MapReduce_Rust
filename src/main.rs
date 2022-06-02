#![allow(unused_imports, dead_code, unused_variables)]
mod container;
mod tasks;
mod threadpool;
mod user_input;

fn main() {
    tasks::map("test_resource/file2.txt".to_string()); 
}
