use std::env;
use std::fs;


fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);
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


fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    Config { query, filename }
}
