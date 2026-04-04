use std::io::{self, Write};

fn convertor() {
    loop {
        print!("Enter the Tempature  ");
        io::stdout().flush().unwrap();

        let mut num = String::new();

        io::stdin().read_line(&mut num).expect("Failed to read");
        println!();
        match num.trim().parse::<f64>() {
            Ok(num) => {
                println!("You entered {}", num);
                break;
            }
            Err(_) => println!("Invalid input"),
        }
    }
}

fn process_imput(letter: &str) {
    println!("Selected '{}' ", letter);
    convertor();
}
fn main() {
    println!("Tempature Convertor");
    loop {
        print!("press f to input Fahrenheit value, c for Celsius or e to end  ");
        io::stdout().flush().unwrap();
        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read");
        let letter = input.trim().to_lowercase();

        match letter.as_str() {
            "f" => process_imput("f"),
            "c" => process_imput("c"),
            "e" => {
                println!("Ending .....");
                break;
            }
            _ => println!("please try again"),
        }
    }
}
