/* Enter a list that presents points of students in a class.
 * The input-ing terminates when user enter a negative number*/
use std::io;

fn input_points(list: &mut Vec<u32>) -> (u32, u32, u32){
    let mut sum: u32 = 0;
    let mut max: u32 = 0;
    loop {
        let mut temp = String::new();
        println!("Enter a number: ");
        io::stdin().read_line(&mut temp).expect("Failed to read line");
        let number:u32 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => break,
        };
        sum = sum + number;
        if max < number {
            max = number;
        }
        list.push(number);
    }
    let ave = sum / (list.len() as u32);
    (sum, ave, max)
}

fn main() {
    let mut list: Vec<u32> = Vec::new();
    let (sum, ave, max) = input_points(&mut list);
    println!("Result: {:?}", list);
    println!("Total points: {} and ave {} with max {}", sum, ave, max);
}
