#![allow(unused_imports, dead_code, unused_variables)]
mod user_input;
mod threadpool; 
mod tasks; 
mod container; 

fn main() {
    // let user_arguments =  user_input::user_arguments::user_args();
    // println!("{}", user_arguments); 
    let contain = container::Container::new(8); 
    let key = "word".to_string(); 
    contain.get_partition(key);
    
}
