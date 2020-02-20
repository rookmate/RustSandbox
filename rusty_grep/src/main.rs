use std::env;
use std::fs;
use std::process;
use std::error::Error;


fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(
        |err| { println!("Could not parse arguments {}", err);
                process::exit(1);
            }
    );

    println!("Filename: {}", config.filename);
    println!("Finding: {}", config.query);

    // Use this instead of unwrap since we don't want to retrieve anything
    if let Err(e) = run(config){
        println!("Error: {}", e);
        process::exit(1);
    };
}

// Basic conf struct
struct Config {
    query: String,
    filename: String,
}

// Init a new conf
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

// Print content from file
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("File content:\n{}", contents);

    Ok(())
}
