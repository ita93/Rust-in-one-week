fn main() {
  let mut t = (12, "eggs");
  let r = t;
  println!("R value: {:?}\n", r);
  println!("T value: {:?}\n", t);
  let b = Box::new(t);  //allocated a tuple
}
