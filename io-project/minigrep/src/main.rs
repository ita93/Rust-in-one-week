extern crate minigrep;
use std::env; //For reading arguments
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    //the args[0] is binary file path, as same as C 
    let config = Config::new(&args).unwrap_or_else(|err| {
      eprintln!("Problem parsing arguments: {}", err);
      process::exit(1);
    });
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);
    //Reading file
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
