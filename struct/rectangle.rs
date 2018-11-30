#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  fn area(&self) -> u32 {
    self.width * self.height
  }

  fn can_hold(&self, other : &Rectangle) -> bool {
    self.width > other.width && self.height > other.height 
  } 
}

fn main() {
  let rect1 = Rectangle{ width: 3, height: 3};
  let rect2 = Rectangle{ width: 10, height: 5};
  println!("Rectangle {:?} area: {}", rect1, rect1.area());
  println!("Can hold rectangle 2? {}", rect1.can_hold(&rect2));
}