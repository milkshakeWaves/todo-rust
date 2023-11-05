use std::env;

mod lib;
mod repository;

fn main() {
    let args: Vec<String> = env::args().collect();
    // Parse arguments and call appropriate functions from lib module
    // ...
    println!("Successful!");
}
