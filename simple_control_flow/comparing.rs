/*
 * Enter 2 integer numbers and compare them
 */
use std::cmp::Ordering;
use std::io;

fn compare(n: i32, m: i32) -> Ordering {
    if n < m {
        Ordering::Less
    } else if n > m {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

fn main() {
    println!("Enter 2 number n and m :");
    let (mut n, mut m) = (String::new(), String::new());
    println!("Enter N: ");
    io::stdin().read_line(&mut n).expect("Failed to read line");
    let n: i32 = n.trim().parse().expect("Please type a number!");

    println!("Enter M: ");
    io::stdin().read_line(&mut m).expect("Failed to read line");
    let m: i32 = m.trim().parse().expect("Please type a number!");
    let res = compare(n, m);
    match res {
        Ordering::Less => println!("N is smaller than M"),
        Ordering::Equal => println!("N is equal to M"),
        Ordering::Greater => println!("N is greater than M")
    }
}
