#![allow(unused_imports, dead_code, unused_variables)]
mod user_input;

fn main() {
    let user_arguments =  user_input::user_arguments::user_args();
    println!("{}", user_arguments); 
}
