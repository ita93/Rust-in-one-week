use std::env; //For reading arguments
use std::fs::File; 
use std::io::prelude::*;

fn main() {
  let args: Vec<String> = env::args().collect();
  //the args[0] is binary file path, as same as C
  let query = &args[1];
  let filename = &args[2];
  
  println!("Searching for {}", query);
  println!("In file {}", filename);
  
  //Reading file
  let mut f = File::open(filename).expect("file not found");
  let mut contents = String::new();
  f.read_to_string(&mut contents).expect("Something went wrong reading the
file");
  println!("With text: \n{}", contents);
}
