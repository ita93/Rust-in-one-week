fn main() {
    let mut v = Vec::new();
    for i in 101..106 {
        v.push(i.to_string());
    }
    
    //let fifth = v.pop().unwrap();

    let third = v.swap_remove(1);
    println!("{:?}", v);
}
