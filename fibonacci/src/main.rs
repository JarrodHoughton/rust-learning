use std::io;

fn main() {
    let mut a: usize = 1;
    let mut b: usize = 1;

    println!("Please enter a number:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let n: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input, using default value of 1000");
            1000
        }
    };

    for _ in 1..n {
        println!("{}", a);
        let c: usize = a + b;
        a = b;
        b = c;
    }
    println!("Fibonacci number {} is {}", n, a);
}
