use std::fs;
use std::error::Error;

// Basic conf struct
pub struct Config {
    pub query: String,
    pub filename: String,
}

// Init a new conf
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

// Print content from file
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &contents){
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
}
