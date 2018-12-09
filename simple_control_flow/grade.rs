/*Enter an integer from keyboard. It must be not lower than 0 and not 
 * higher than 10. Classify this grade based on following rules:
 * If point < 5 -> Fail
 * 5<=point<=7: Fair
 * 8<=point<=9: Good
 * point = 10: Very good.
 */
use std::io;
use std::fmt;

#[derive(Debug)]
enum Grade {
    Fail,
    Fair,
    Good,
    VeryGood,
    None
}

/*implement display strait for Grade*/
impl fmt::Display for Grade {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Grade::Fail => write!(f,"You cannot pass this course!"),
            Grade::Fair => write!(f, "You passed it!!!"),
            Grade::Good => write!(f, "You passed it with high point!"),
            Grade::VeryGood => write!(f, "You are perfect!"),
            Grade::None => write!(f, "Hmmmm")
        }
    }
}

fn classify(p: u32) -> Grade {
    if p < 5 {
        Grade::Fail
    } else if p>=5 && p<=7 {
        Grade::Fair
    } else if p>=8 && p<=9 {
        Grade::Good
    } else if p==10 {
        Grade::VeryGood
    } else {
        Grade::None
    }

}

fn main(){
    println!("Please input your point.");
    let mut point_str = String::new();
    io::stdin().read_line(&mut point_str);
    let point:u32 = point_str.trim().parse().expect("Please type a number!");
    let result = classify(point);
    println!("Student type: {}", result);
}
