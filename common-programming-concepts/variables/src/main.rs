use std::io;
use std::env;

// const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

// mutability
// fn main() {
//     let mut x = 5;
//     println!("The value of x is: {}", x);
//     x = 7;
//     println!("The value of x is: {}", x);
//
//     println!("The value of THREE_HOURS_IN_SECONDS is: {}", THREE_HOURS_IN_SECONDS);
// }

// shadowing
// fn main() {
//     let x = 5;
//
//     let x = x + 1;
//
//     {
//         let x = x * 2;
//         println!("The value of x in the inner scope is: {x}");
//     }
//
//     println!("The value of x is: {x}");
// }
// The value of x in the inner scope is: 12
// The value of x is: 6

// tuples
// fn main() {
//     let tup = (500, 6.4, 1);
//
//     // let (x, y, z) = tup; // destructuring the tuple
//
//     // println!("The value of y is: {y}");
//     println!("value 0: {}", tup.0);
//
//     println!("value 1: {}", tup.1);
//
//     println!("value 2: {}", tup.2);
// }

// arrays
fn main() {
    // env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RUST_BACKTRACE", "full");

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
