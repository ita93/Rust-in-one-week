struct User{
  name : String,
  email : String,
  age : u32,
}

fn main(){
  let phi = User{
    name : String::from("Phi Nguyen"),
    email : String::from("phi_nguyen@compex.com.sg"),
    age : 20,
  };
  println!("Hello world\n"); 
}
