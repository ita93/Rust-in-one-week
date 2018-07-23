/*Data types in Rust
 * Rust - statically type language -> It need to know 
 * the data type of variables at compile time.
 * Compiler usally guesses types base on variable's value
 * and usage. If there are many valid data type, we must
 * add a data type annotation.
 * Two subsets:
 * 1. scalar - presents a single value (int, float, ....)
 *  a. Integer:
 *      ux : x is a number (multiple of 8) - an unsigned x bits integer.
 *      ix : x is a number (multiple of 8) - a signed x bits integer.
 *  b. Floating-point:
 *      f32-single precision or f64(default)-double precision
 *  c. Boolean: true or false. Rust won't auto conver non-boolean to boolean type.
 *  d. Character char type: round with ''
 * 2. compbound: Group multiple value into one type.
 *  a. Tupple type: Way to group some number of other value of variant
 *  type into one compbound type.
 *  b. Array: Similar with tupe, but every element must be the same type.
 *      arrays in rust are fix length, once declare, length cannot be changed.
 *      Rust allocate array in stack instead of head.
 */
fn main() {
    println!("Hello, world!");
    let number: u32 = "42".parse().expect("not a number");
    let sum = 9.0 + 9.5; //floating-point.
    let my_char = 'P'; //
    println!("X: {}",number);
    println!("Floating-point sum {}", sum);
    println!("Char type example: {}", my_char);
    /*Tupple type*/
    let my_tup: (i32, f64, u8) = (60,50.0,40);  //create a tup.
    let (x,y,z) = my_tup;                       //destructuring a tup.
    println!("Second element: {}",y);           //print second element of my_tup
    println!("Second element through index {}",my_tup.1);           //print second element of my_tup
    let arr = [1,2,3,4,5];                      //Array.
}
