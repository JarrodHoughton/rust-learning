use std::io;

fn main() {
    let mut a: u128 = 1;
    let mut b: u128 = 1;

    println!("Please enter a number:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let n: u128 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input, using default value of 1000");
            1000
        }
    };

    for _ in 1..n {
        println!("{}", a);
        let c: u128 = a + b;
        a = b;
        b = c;
    }
    println!("Fibonacci number {} is {}", n, a);
}
