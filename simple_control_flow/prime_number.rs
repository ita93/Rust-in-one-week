/*Enter a positive integer and check wether it is a prime number or not*/
use std::io;

fn check_prime() -> bool {
    println!("Enter your number here: ");
    let mut temp:String = String::new();
    io::stdin().read_line(&mut temp).expect("failed to read line");
    let n: u32 = match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Not a positive number")
    };
    let haft_n = n / 2;
    for i in 2..=haft_n {
        if n % (i as u32) == 0 {
            return false
        }
    }
    true
}

fn main() {
    let res = check_prime();
    println!("{}", res);
}
