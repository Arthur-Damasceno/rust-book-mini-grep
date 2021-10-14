mod config;

pub use config::Config;
use std::fs;

pub fn run(config: Config) {
    println!(
        "Searching for: {}\nIn file: {}\n",
        config.query, config.filename
    );

    let contents =
        fs::read_to_string(config.filename).expect("Something went wrong reading the file");

    println!("With the context:\n{}", contents)
}
