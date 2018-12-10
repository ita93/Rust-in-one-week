/*Enter an integer and return the sum of all odds thoese lesser than that one*/
use std::io;

fn odds_sum() -> u32 {
    println!("Enter your decated number");
    let mut temp: String = String::new();
    io::stdin().read_line(&mut temp).expect("Cannot read line");
    let entered_number:u32 = match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Not a number")
    };

    let mut sum:u32 = 0;

    for i in 1..=entered_number {
        if (i as u32) % 2 != 0 {
            sum += i as u32;
        }
    }

    sum
}

fn main() {
    let result = odds_sum();
    println!("The result: {}", result);
}
