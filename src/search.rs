pub struct Search;

impl Search {
    pub fn case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        let mut results = Vec::new();

        for line in contents.lines() {
            if line.contains(query) {
                results.push(line);
            }
        }

        results
    }

    pub fn case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        let query = query.to_lowercase();
        let mut results = Vec::new();

        for line in contents.lines() {
            if line.to_lowercase().contains(&query) {
                results.push(line)
            }
        }

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            Search::case_sensitive(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            Search::case_insensitive(query, contents)
        );
    }
}
