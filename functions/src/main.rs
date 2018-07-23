fn main() {
    let num = 5;
    //funny if
    let res = if num == 5 {
        10
    }else{
        20
    };
    println!("RES: {}", res);
    if num < 5{
        println!("Hello, world!");
    }else{
        let sum = another_function(6,8); //not like C, you can decalre-define your function after caller.
        println!("Sum of x and y: {}",sum);
    }
    //Loop
    println!("LOOP FOREVER:");
    let mut count=5;
    loop{
        println!("Current counter: {}", count);
        count = count - 1;
        if count == 0{
            break;
        };
    };
    
    println!("WHILE LOOP:");
    let mut count=5;
    while count != 0{
        println!("Current counter: {}", count);
        count = count - 1;
    };
    
    println!("FOR LOOP:");
    for number in (1..5){
        println!("Value {}", number);
    };
}

/*
 * Rust convention: use snake_case
 */
fn another_function(x: i32, y: i32) -> i32{//Need to specify argument's type in signature.
    println!("Another function with X={} and Y={}",x,y);
    x+y //this line means return value - return (x+y);
}
