/* Enter a list that presents points of students in a class.
 * The input-ing terminates when user enter a negative number*/
use std::io;

fn input_points(list: &mut Vec<u32>){
    loop {
        let mut temp = String::new();
        println!("Enter a number: ");
        io::stdin().read_line(&mut temp).expect("Failed to read line");
        let number:u32 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => break,
        };
        list.push(number);
    }
}

fn main() {
    let mut list: Vec<u32> = Vec::new();
    input_points(&mut list);
    println!("Result: {:?}", list);
}
