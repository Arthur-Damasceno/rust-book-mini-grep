mod config;
mod search;

pub use config::Config;
use search::search;
use std::{error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!(
        "Searching for: {}\nIn file: {}\n",
        config.query, config.filename
    );

    let contents = fs::read_to_string(config.filename)?;

    println!("Results:");
    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    println!("\nWith the context:\n{}", contents);

    Ok(())
}
