#[derive(Debug)]
enum IpAddr {
  V4(String),
  V6(String),
}

impl IpAddr {
  fn get_address(&self) {
    println!("Ip address: {:?}", self);
  }
}

fn main() {
  let home = IpAddr::V4(String::from("127.0.0.1"));
  let loopback = IpAddr::V6(String::from("::1"));
  println!("Home ip address: {:?}", home);
  println!("Loopback ip address: {:?}", loopback);
  home.get_address();
}
