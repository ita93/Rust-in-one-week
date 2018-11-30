struct Point {
    x: f32,
    y: f32,
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    fn area(&self) -> f32 {
        let Rectangle {top_left: Point {x: x1, y: y1}, bottom_right: Point {x: x2, y: y2}} = self;
        let length = x2 - x1;
        let height = y1 - y2;
        length * height
    }
}

fn main() {
    let rect = Rectangle {top_left: Point {x: 2.0, y: 10.0}, bottom_right: Point {x: 10.0, y: 2.0}};
    println!("Area of rectangle: {}", rect.area());
}
