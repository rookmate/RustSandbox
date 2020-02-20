use std::env;
use std::fs;
use std::process;


fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(
        |err| { println!("Could not parse arguments {}", err);
                process::exit(1);
            }
    );

    println!("Filename: {}", config.filename);
    println!("Finding: {}", config.query);

    let contents = fs::read_to_string(config.filename)
        .expect("Could not read file");

    println!("File content:\n{}", contents);
}


struct Config {
    query: String,
    filename: String,
}


impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
