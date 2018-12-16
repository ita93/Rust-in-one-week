fn main() {
  println!("Nguyen DInh Phi
2011993");
  let data = "initial contents";
  let s1 = data.to_string();
  let s2 = "initial contents".to_string();
  let s3 = String::from("initial contents");
  let mut s4 = String::from("foo");
  s4.push_str("bar");
  println!("s4 is {}", s4);

  let mut s5 = String::from("foo");
  let s6 = "bar";
  s5.push_str(s6);
  println!("s6 is {}", s5);
}
