mod config;

use std::{error::Error, fs};
pub use config::Config;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!(
        "Searching for: {}\nIn file: {}\n",
        config.query, config.filename
    );

    let contents = fs::read_to_string(config.filename)?;

    println!("With the context:\n{}", contents);

    Ok(())
}
