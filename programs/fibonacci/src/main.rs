use std::io;

fn calculate_fibonacci_nth(n: u32) -> u32 {
    let mut a: u32 = 0;
    let mut b: u32 = 1;
    let mut temp: u32 = a + b;

    let mut counter: u32 = 1;

    while counter < n {
        temp = a + b;
        a = b;
        b = temp;
        counter += 1;
    }

    temp
}

fn main() {
    let mut fibonacci_nth = String::new();

    println!("Enter the nth fibonacci number: ");

    io::stdin()
        .read_line(&mut fibonacci_nth)
        .expect("Failed to read line");

    let fibonacci_nth: u32 = fibonacci_nth.trim().parse().expect("Please type a number!");

    println!(
        "The {}th fibonacci number is {}",
        fibonacci_nth,
        calculate_fibonacci_nth(fibonacci_nth)
    );
}
