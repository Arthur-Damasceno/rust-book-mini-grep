use std::{env, fs};

pub fn run() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for: {}\nIn file: {}\n", query, filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("With the context:\n{}", contents)
}
