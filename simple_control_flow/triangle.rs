use std::io;

/* Enter 3 numbers and check if they are veres of a triangle*/

fn main() {
    println!("Enter three number a, b, c");
    println!("Enter a: ");

    let mut temp = String::new();
    io::stdin().read_line(&mut temp).expect("Failed to read line");
    let a:u32 = temp.trim().parse().expect("Please enter a unsigned numer");

    temp = String::new();
    io::stdin().read_line(&mut temp).expect("Failed to read line");
    let b:u32 = temp.trim().parse().expect("Please enter a unsigned numer");

    temp = String::new();
    io::stdin().read_line(&mut temp).expect("Failed to read line");
    let c:u32 = temp.trim().parse().expect("Please enter a unsigned numer");
    
    if a > b+c || b > a+c || c > a+b {
        println!("This is not a triangle");
    } else {
        println!("This is a triangle");
    }
}
