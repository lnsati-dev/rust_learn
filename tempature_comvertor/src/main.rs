use std::io::{self, Write};

fn convertor() -> f64 {
    loop {
        print!("Enter the Tempature  ");
        io::stdout().flush().unwrap();

        let mut num = String::new();

        io::stdin().read_line(&mut num).expect("Failed to read");
        println!();
        match num.trim().parse::<f64>() {
            Ok(num) => return num,
            Err(_) => println!("Invalid input"),
        }
    }
}

fn process_imput(letter: &str) {
    println!("Selected '{}' ", letter);
    if letter == "c" {
        let numt = convertor();
        let numf = (numt * SF) + FP;
        println!("This is {:.1}'F", numf);
    } else {
        let numt = convertor();
        let numc = (numt - FP) * ISF;
        println!("This is {:.1}'C", numc);
    }
}

const FP: f64 = 32.0;
const SF: f64 = 9.0 / 5.0;
const ISF: f64 = 5.0 / 9.0;

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
