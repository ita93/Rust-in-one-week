use std::env; //For reading arguments
//use std::fs::File; 
//use std::io::prelude::*;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    //the args[0] is binary file path, as same as C 
    let config = parse_config(&args); 
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);
    //Reading file
    let contents = fs::read_to_string(config.filename).expect("Something went wrong reading the file");
    println!("With text: \n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    Config {query, filename}
}
