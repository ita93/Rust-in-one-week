enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main()
{
    let my_coin = Coin::Dime;
    match my_coin {
        Coin::Penny => println!("Penny") ,
        Coin::Nickel => println!("Nickel") ,
        Coin::Dime => println!("Dime") ,
        Coin::Quarter => println!("Quarter") ,
    };
}
