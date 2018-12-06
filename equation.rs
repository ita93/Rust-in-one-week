use std::io;

fn main() {
  let mut input: String = String::new();
  
  println!("Input A: ");
  io::stdin().read_line(&mut input).ok().expect("Failed to read line");
  
  let a: i32 = match input.trim().parse() {
    Ok(x) => x,
    Err(_) => panic!("Invalid value for A"),
  };
  
  if a == 0 {
    panic!("A must be different than 0");
  }
  
  input = String::new();
  println!("Input B: ");
  io::stdin().read_line(&mut input).ok().expect("Failed to read line");
  let b: i32 = match input.trim().parse() {
    Ok(x) => x,
    Err(_) => panic!("Invalid value for B"),
  };
  
  //input = String::new();
  println!("Input C: ");
  io::stdin().read_line(&mut input).ok().expect("Failed to read line");
}
