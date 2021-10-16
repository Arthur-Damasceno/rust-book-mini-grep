mod config;
mod search;

pub use config::Config;
use search::Search;
use std::{error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!(
        "Searching for: {}\nIn file: {}\n",
        config.query, config.filename
    );

    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        Search::case_sensitive(&config.query, &contents)
    } else {
        Search::case_insensitive(&config.query, &contents)
    };

    println!("Results:");
    for line in results {
        println!("{}", line);
    }

    println!("\nWith the context:\n{}", contents);

    Ok(())
}
