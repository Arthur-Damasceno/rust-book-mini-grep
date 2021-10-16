use std::env;

#[derive(Debug, PartialEq)]
pub struct Config<'a> {
    pub query: &'a str,
    pub filename: &'a str,
    pub case_sensitive: bool,
}

impl<'a> Config<'a> {
    pub fn new(args: &Vec<String>) -> Result<Config, &'static str> {
        let mut args = args.iter();

        let query = match args.next() {
            Some(query) => query,
            None => return Err("Didn't get a query"),
        };

        let filename = match args.next() {
            Some(filename) => filename,
            None => return Err("Didn't get a filename"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_new() {
        let args = vec![
            String::new(),
            String::from("test"),
            String::from("test.txt"),
        ];
        let config = Config::new(&args).unwrap();

        assert_eq!(config.query, &args[1]);
        assert_eq!(config.filename, &args[2]);
    }

    #[test]
    fn config_new_with_invalid_args() {
        let args = vec![String::new()];
        let config = Config::new(&args);

        assert_eq!(config, Err("Not enough arguments"));
    }
}
