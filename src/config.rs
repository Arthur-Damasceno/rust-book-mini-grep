use std::env;

#[derive(Debug, PartialEq)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() != 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

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

        assert_eq!(config.query, args[1]);
        assert_eq!(config.filename, args[2]);
    }

    #[test]
    fn config_new_with_invalid_args() {
        let args = vec![String::new()];
        let config = Config::new(&args);

        assert_eq!(config, Err("Not enough arguments"));
    }
}
