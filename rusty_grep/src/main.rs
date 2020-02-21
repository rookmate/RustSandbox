use std::env;
use std::process;

use rusty_grep::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(
        |err| { eprintln!("Could not parse arguments {}", err);
                process::exit(1);
            }
    );

    println!("Filename: {}", config.filename);
    println!("Finding: {}", config.query);

    // Use this instead of unwrap since we don't want to retrieve anything
    if let Err(e) = rusty_grep::run(config){
        eprintln!("Error: {}", e);
        process::exit(1);
    };
}
