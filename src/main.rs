#![allow(unused_imports, dead_code, unused_variables)]
mod container;
mod tasks;
mod threadpool;
mod user_input;

fn main() {
    let mut container = container::Container::new(10);
    tasks::map("test_resource/file1.txt".to_string(), &mut container);
    for i in 0..container.get_size() {
        println!("-----------------------------------------------------------------------");
        println!("partition : {}", i);
        tasks::reducer(i, &mut container);
        println!("-----------------------------------------------------------------------");
    }
}
