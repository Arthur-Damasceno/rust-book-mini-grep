use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for: {}\nIn file: {}", query, filename);
}
