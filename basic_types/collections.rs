fn main() {
  let lazy_caterer: [u32; 6] = [1, 2, 4, 7, 11, 16];
  let taxonomy = ["Animalia", "Arthropoda", "Insecta"];

  assert_eq!(lazy_caterer[3], 7);
  assert_eq!(taxonomy.len(), 3);

  let mut chaos = [1,3,2,4,5];
  chaos.sort();
  println!("CHaos array {:?}", chaos);
}
