use std::fs;
use std::env;
use std::error::Error;

// Basic conf struct
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

// Init a new conf
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        // Checks value of env var from the command line
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}

// Print content from file
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

// Search file for string
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

// Search case insensitive
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "huc";
        let contents = "
Chuck Testa
Rust unit-tests
FeelsGoodMan";

        assert_eq!(
            vec!["Chuck Testa"],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "Huc";
        let contents = "
Chuck Testa
Rust unit-tests
FeelsGoodMan
Huckleberry Trust";

        assert_eq!(
            vec!["Chuck Testa", "Huckleberry Trust"],
            search_case_insensitive(query, contents)
        )
    }
}
