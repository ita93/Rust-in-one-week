/* THis problem is too popular*/
use std::io;

fn fibonacci_n(number: u32) -> u32 {
    let res = match number {
        0 => 0,
        1 => 1,
        _ => {
            let (mut sum, mut prev1, mut prev2, mut i) = (0, 0, 1, 2);
            while i <= number {
                sum = prev1 + prev2;
                prev1 = prev2;
                prev2 = sum;
                i += 1;
            }
            sum
        }
    };
    res
}

fn main() {
    let mut temp = String::new();
    io::stdin().read_line(&mut temp).expect("Read line failed");
    let num:u32 = temp.trim().parse().expect("Please enter an integer");
    let res = fibonacci_n(num);
    println!("The fibonnaci {}(st,nd,rd,th) is {}", num, res);
}
