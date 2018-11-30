#[derive(Debug)]
struct Person<'a> {
  name: &'a str,
  age: u8,
}

//Unit struct
struct Nil;

//a tuple struct
struct Pair(i32, f32);

//a struct with two fields
struct Point{
  x: f32,
  y: f32,
}

#[allow(dead_code)]
struct Rectangle{
  p1: Point,
  p2: Point,
}

fn main(){
  //Create struct with field init shorthand
  let name = "Peter";
  let age = 27;
  let peter = Person { age, name };
  
  //Print debug struct
  println!("{:?}", peter);

  //Instantiate a Point
  let point: Point = Point { x: 0.3, y: 0.4};
  
  //Access the fields of the point
  println!("Point coordinates: ({}, {})", point.x, point.y);

  //Destructure the point using a `let` binding
  let Point { x: my_x, y: my_y } = point;
  println!("Destructing {} and {} ", my_x, my_y);

  let _rectangle = Rectangle {
    //struct instantiation is an express too
    p1: Point { x: my_y, y: my_x },
    p2: point,
  };

  let _nil = Nil;
  
  let pair = Pair(1, 0.1);
  
  println!("pair contains {:?} and {:?}", pair.0, pair.1);
  
  let Pair(integer, decimal) = pair;
  
  println!("pair contains {:?} and {:?}", integer, decimal);
}
