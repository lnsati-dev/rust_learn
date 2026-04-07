use std::io::{self, Write};

fn fibonacci(n: u32) -> u128 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    let mut a: u128 = 0;
    let mut b: u128 = 1;

    for _ in 2..=n {
        let next = a + b;
        a = b;
        b = next;
    }
    b
}

fn main() {
    println!("##### Fibonacci Sequence Generator #####");
    println!("'n' must be between 0 and 186. \n");

    let n: u32 = loop {
        print!("Enter the value of n ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();
        match input.parse::<u32>() {
            Ok(num) if num <= 186 => break num,
            Ok(_) => println!("Error: value too large."),
            Err(_) => println!("Error: invalid input. "),
        }
    };
    let result = fibonacci(n);
    println!("The {}th Fibonacci number is: {} \n", n, result);
}
