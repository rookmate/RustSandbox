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

    println!("File content:\n{}", contents);

    Ok(())
}
