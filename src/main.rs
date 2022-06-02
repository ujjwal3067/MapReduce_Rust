#![allow(unused_imports, dead_code, unused_variables)]
mod container;
mod tasks;
mod threadpool;
mod user_input;

fn main() {
    // let user_arguments =  user_input::user_arguments::user_args();
    // println!("{}", user_arguments);
    let mut contain = container::Container::new(8);
    let key = "word".to_string();
    contain.insert_pair(container::Pair::new(key, 2));
    println!("{:?}", contain);
}
